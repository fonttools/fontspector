use std::collections::HashMap;

use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::{raw::TableProvider, string::StringId, MetadataProvider};

#[check(
    id = "monotype/fvar_STAT_axis_names",
    rationale = "
        fvar and STAT should list the same axis tags and use consistent name-table
        strings for each shared axis. If the resolved axis names differ, applications
        may show different labels for the same design axis.
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/36",
    title = "Does each axis name match between fvar and STAT?"
)]
#[allow(non_snake_case)]
fn fvar_STAT_axis_names(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font"
    );

    let Ok(stat) = f.font().stat() else {
        return Ok(Status::just_one_skip(
            "no-stat",
            "STAT table not found; cannot compare axis names with fvar.",
        ));
    };

    let stat_name_id_by_tag: HashMap<String, StringId> = stat
        .design_axes()?
        .iter()
        .map(|axis| (axis.axis_tag().to_string(), axis.axis_name_id()))
        .collect();

    let mut problems = vec![];

    for fvar_axis in f.font().axes().iter() {
        let tag = fvar_axis.tag().to_string();
        let Some(stat_name_id) = stat_name_id_by_tag.get(&tag) else {
            continue;
        };

        let fvar_name_id = fvar_axis.name_id();
        let fvar_name = f.get_best_name(&[fvar_name_id]);
        let stat_name = f.get_best_name(&[*stat_name_id]);

        if fvar_name.is_none() {
            problems.push(Status::fail(
                "fvar-axis-name-missing-in-name-table",
                &format!(
                    "Axis '{tag}' fvar axisNameID {fvar_id} does not resolve to a name string.",
                    tag = tag,
                    fvar_id = fvar_name_id.to_u16(),
                ),
            ));
        }
        if stat_name.is_none() {
            problems.push(Status::fail(
                "stat-axis-name-missing-in-name-table",
                &format!(
                    "Axis '{tag}' STAT axisNameID {stat_id} does not resolve to a name string.",
                    tag = tag,
                    stat_id = stat_name_id.to_u16(),
                ),
            ));
        }
        if let (Some(fvar_name), Some(stat_name)) = (&fvar_name, &stat_name) {
            if fvar_name != stat_name {
                problems.push(Status::fail(
                    "fvar-stat-axis-name-mismatch",
                    &format!(
                        "Axis '{tag}' name mismatch between fvar ({fvar_name:?}) and STAT ({stat_name:?}).",
                        tag = tag,
                        fvar_name = fvar_name,
                        stat_name = stat_name,
                    ),
                ));
            }
        }
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

    use super::fvar_STAT_axis_names;
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, assert_skip, run_check, test_able,
    };
    use fontspector_checkapi::{FileTypeConvert, StatusCode, TTF};
    use skrifa::raw::TableProvider;
    use write_fonts::{
        from_obj::ToOwnedTable,
        tables::{fvar::Fvar, stat::Stat},
        types::NameId,
    };

    #[test]
    fn test_fvar_stat_axis_names_skip_non_variable() {
        let testable = test_able("montserrat/Montserrat-Black.ttf");
        let results = run_check(fvar_STAT_axis_names, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_fvar_stat_axis_names_pass() {
        let testable = test_able("varfont/OpenSans[wdth,wght].ttf");
        let results = run_check(fvar_STAT_axis_names, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fvar_stat_axis_names_mismatch() {
        let mut testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut stat: Stat = f.font().stat().unwrap().to_owned_table();
        stat.design_axes[0].axis_name_id = NameId::new(2);
        testable.set(f.rebuild_with_new_table(&stat).unwrap());

        let results = run_check(fvar_STAT_axis_names, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("fvar-stat-axis-name-mismatch".to_string()),
        );
    }

    #[test]
    fn test_fvar_stat_axis_names_missing_both_names() {
        let mut testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();
        fvar.axis_instance_arrays.axes[0].axis_name_id = NameId::new(36000);
        testable.set(f.rebuild_with_new_table(&fvar).unwrap());

        let f = TTF.from_testable(&testable).unwrap();
        let mut stat: Stat = f.font().stat().unwrap().to_owned_table();
        stat.design_axes[0].axis_name_id = NameId::new(36001);
        testable.set(f.rebuild_with_new_table(&stat).unwrap());

        let results = run_check(fvar_STAT_axis_names, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("fvar-axis-name-missing-in-name-table".to_string()),
        );
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("stat-axis-name-missing-in-name-table".to_string()),
        );
    }

    #[test]
    fn test_fvar_stat_axis_names_missing_stat_name() {
        let mut testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut stat: Stat = f.font().stat().unwrap().to_owned_table();
        stat.design_axes[0].axis_name_id = NameId::new(36000);
        testable.set(f.rebuild_with_new_table(&stat).unwrap());

        let results = run_check(fvar_STAT_axis_names, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("stat-axis-name-missing-in-name-table".to_string()),
        );
    }
}

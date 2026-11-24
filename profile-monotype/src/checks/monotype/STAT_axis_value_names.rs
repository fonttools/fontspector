use std::collections::HashMap;

use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::raw::TableProvider;

#[check(
    id = "monotype/STAT_axis_value_names",
    rationale = "
        STAT axis value records each carry a valueNameID. Resolved name strings
        should be unique across the STAT axis values subtable (global within STAT).
        Duplicate labels create ambiguous entries in application font menus.

        Format 3 linkedValue is a numeric style-link target only and does not
        contribute a second name; the format 3 record's own valueNameID is still
        counted like any other axis value name.
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/37",
    title = "Are STAT axisValue names unique within STAT?"
)]
#[allow(non_snake_case)]
fn STAT_axis_value_names(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let Ok(stat) = f.font().stat() else {
        skip!("no-stat", "STAT table not found");
    };
    let Some(Ok(subtable)) = stat.offset_to_axis_values() else {
        return Ok(Status::just_one_skip(
            "no-axis-values",
            "STAT table has no axis values",
        ));
    };

    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for axis_value in subtable.axis_values().iter().flatten() {
        let Some(name) = f.get_best_name(&[axis_value.value_name_id()]) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        *name_counts.entry(name).or_insert(0) += 1;
    }

    let mut problems = vec![];
    for (name, count) in name_counts {
        if count > 1 {
            problems.push(Status::fail(
                "duplicate-stat-axis-value-name",
                &format!(
                    "STAT axis value name '{name}' is used {count} times. \
                     Duplicate names create ambiguous labels in font menus."
                ),
            ));
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

    use super::STAT_axis_value_names;
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, assert_skip, run_check, test_able,
    };
    use fontspector_checkapi::{FileTypeConvert, StatusCode, TTF};
    use skrifa::raw::TableProvider;
    use write_fonts::{
        from_obj::ToOwnedTable,
        tables::stat::{AxisValue as WriteAxisValue, Stat},
        types::NameId,
    };

    fn set_all_axis_value_name_ids(stat: &mut Stat, name_id: NameId) {
        if let Some(values) = stat.offset_to_axis_values.as_mut() {
            for value in values.iter_mut() {
                match &mut **value {
                    WriteAxisValue::Format1(v) => v.value_name_id = name_id,
                    WriteAxisValue::Format2(v) => v.value_name_id = name_id,
                    WriteAxisValue::Format3(v) => v.value_name_id = name_id,
                    WriteAxisValue::Format4(v) => v.value_name_id = name_id,
                }
            }
        }
    }

    #[test]
    fn test_stat_axis_value_names_skip_no_stat() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let results = run_check(STAT_axis_value_names, testable);
        assert_results_contain(&results, StatusCode::Skip, Some("no-stat".to_string()));
    }

    #[test]
    fn test_stat_axis_value_names_pass() {
        let testable = test_able("varfont/OpenSans[wdth,wght].ttf");
        let results = run_check(STAT_axis_value_names, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_stat_axis_value_names_duplicate() {
        let mut testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut stat: Stat = f.font().stat().unwrap().to_owned_table();
        // Force every axis value to share one existing nameID so names collide.
        let shared_name_id = match &**stat.offset_to_axis_values.as_ref().unwrap().first().unwrap()
        {
            WriteAxisValue::Format1(v) => v.value_name_id,
            WriteAxisValue::Format2(v) => v.value_name_id,
            WriteAxisValue::Format3(v) => v.value_name_id,
            WriteAxisValue::Format4(v) => v.value_name_id,
        };
        set_all_axis_value_name_ids(&mut stat, shared_name_id);
        testable.set(f.rebuild_with_new_table(&stat).unwrap());

        let results = run_check(STAT_axis_value_names, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("duplicate-stat-axis-value-name".to_string()),
        );
    }

    #[test]
    fn test_stat_axis_value_names_skip_no_axis_values() {
        let mut testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut stat: Stat = f.font().stat().unwrap().to_owned_table();
        stat.offset_to_axis_values = Default::default();
        testable.set(f.rebuild_with_new_table(&stat).unwrap());

        let results = run_check(STAT_axis_value_names, testable);
        assert_skip(&results);
        assert_results_contain(
            &results,
            StatusCode::Skip,
            Some("no-axis-values".to_string()),
        );
    }
}

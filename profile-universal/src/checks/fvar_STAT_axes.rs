use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::{raw::TableProvider, MetadataProvider};

#[check(
    id = "fvar_stat_axes",
    rationale = "The purpose of this check is to make sure that all axes referenced in the fvar table do exist in the STAT table.",
    proposal = "https://github.com/Monotype/fontspector/issues/20",
    title = "All axes referenced by fvar instances exist in the STAT table?"
)]
fn fvar_stat_axes(t: &Testable, _context: &Context) -> CheckFnResult {
    let mut problems: Vec<Status> = vec![];
    let f = testfont!(t);
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font"
    );

    if !f.has_table(b"STAT") {
        problems.push(Status::fail(
            "missing-stat-table",
            "A STAT table should be present in a variable font",
        ));
    } else {
        let stat = f.font().stat()?;
        let stat_axis_tags = stat
            .design_axes()?
            .iter()
            .map(|axis| axis.axis_tag().to_string())
            .collect::<Vec<_>>();

        for axis in f.font().axes().iter() {
            if !stat_axis_tags.contains(&axis.tag().to_string()) {
                problems.push(Status::fail(
                    "missing-stat-axis",
                    &format!(
                        "Axis '{}' is defined in fvar but missing in STAT table",
                        axis.tag()
                    ),
                ));
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use super::fvar_stat_axes;
    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_pass, assert_results_contain, assert_skip,
            remove_table, run_check, test_able,
        },
        FileTypeConvert, StatusCode,
    };

    use write_fonts::{
        tables::fvar::{Fvar, VariationAxisRecord},
        types::{Fixed, Tag},
    };

    #[test]
    fn test_fvar_stat_axes_skip() {
        let testable = test_able("montserrat/Montserrat-Black.ttf");
        let results = run_check(fvar_stat_axes, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_fvar_stat_axes_pass() {
        let testable = test_able("varfont/OpenSans[wdth,wght].ttf");
        let results = run_check(fvar_stat_axes, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fvar_stat_axes_fail_missing_STAT() {
        let mut testable = test_able("varfont/OpenSans[wdth,wght].ttf");
        remove_table(&mut testable, b"STAT");
        let results = run_check(fvar_stat_axes, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("missing-stat-table".to_string()),
        );
    }

    #[test]
    fn test_fvar_stat_axes_fail() {
        let mut testable = test_able("varfont/OpenSans[wdth,wght].ttf");

        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();

        // create a new fvar table with the desired axes
        let mut fvar_new = Fvar::default();
        fvar_new
            .axis_instance_arrays
            .axes
            .push(VariationAxisRecord {
                axis_tag: Tag::new(b"opsz"),
                min_value: Fixed::from_i32(6),
                default_value: Fixed::from_i32(12),
                max_value: Fixed::from_i32(72),
                ..Default::default()
            });
        fvar_new
            .axis_instance_arrays
            .axes
            .push(VariationAxisRecord {
                axis_tag: Tag::new(b"wdth"),
                min_value: Fixed::from_f64(75.0),
                default_value: Fixed::from_f64(100.0),
                max_value: Fixed::from_f64(125.0),
                ..Default::default()
            });

        testable.set(f.rebuild_with_new_table(&fvar_new).unwrap());

        let results = run_check(fvar_stat_axes, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("missing-stat-axis".to_string()),
        );
        assert_messages_contain(
            &results,
            "Axis 'opsz' is defined in fvar but missing in STAT table",
        );
    }
}

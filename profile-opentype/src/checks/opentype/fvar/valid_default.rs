use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "opentype/fvar/valid_default",
    title = "Validates that fvar axis default values are within the axis range.",
    rationale = "
        According to the OpenType specification, the default value of each
        axis defined in the fvar table must be within the range defined by
        the axis minimum and maximum values.

        A default value outside the axis range indicates a malformed font
        that may cause unpredictable behavior in applications.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/176"
)]
fn valid_default(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font."
    );
    let mut problems = vec![];
    for axis in f.font().axes().iter() {
        let tag = axis.tag();
        let min = axis.min_value();
        let default = axis.default_value();
        let max = axis.max_value();
        if default < min || default > max {
            problems.push(Status::fail(
                "invalid-default",
                &format!(
                    "The default value ({}) for the '{}' axis is outside the range [{}, {}].",
                    default, tag, min, max
                ),
            ));
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontations::{
        skrifa::raw::TableProvider,
        write::{from_obj::ToOwnedTable, tables::fvar::Fvar, FontBuilder},
    };
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, assert_skip, run_check, test_able},
        FileTypeConvert, StatusCode, TTF,
    };

    use super::valid_default;

    #[test]
    fn test_inter_passes() {
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(valid_default, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_skip_static() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(valid_default, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_default_below_min() {
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();

        // Set wght default below min
        for axis in &mut fvar.axis_instance_arrays.axes {
            if axis.axis_tag == fontations::write::types::Tag::new(b"wght") {
                axis.default_value = fontations::write::types::Fixed::from_f64(50.0);
                // min is 100, so 50 is below
            }
        }

        let new_bytes = FontBuilder::new()
            .add_table(&fvar)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();
        testable.contents = new_bytes;

        let results = run_check(valid_default, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-default".to_string()),
        );
    }

    #[test]
    fn test_default_above_max() {
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();

        // Set wght default above max
        for axis in &mut fvar.axis_instance_arrays.axes {
            if axis.axis_tag == fontations::write::types::Tag::new(b"wght") {
                axis.default_value = fontations::write::types::Fixed::from_f64(1100.0);
                // max is 900, so 1100 is above
            }
        }

        let new_bytes = FontBuilder::new()
            .add_table(&fvar)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();
        testable.contents = new_bytes;

        let results = run_check(valid_default, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-default".to_string()),
        );
    }
}

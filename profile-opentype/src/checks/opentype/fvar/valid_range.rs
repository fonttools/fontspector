use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "opentype/fvar/valid_range",
    title = "Validates that fvar axis maxValue is greater than minValue.",
    rationale = "
        Each axis defined in the fvar table must have a maximum value
        strictly greater than its minimum value. An axis where maxValue
        is less than or equal to minValue is degenerate — it defines no
        usable variation range and likely indicates a build error such
        as a single-master designspace being compiled as a variable font.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/177"
)]
fn valid_range(t: &Testable, _context: &Context) -> CheckFnResult {
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
        let max = axis.max_value();
        if max <= min {
            problems.push(Status::fail(
                "invalid-range",
                &format!(
                    "The '{}' axis has maxValue ({}) <= minValue ({}). \
                     This defines no usable variation range.",
                    tag, max, min
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

    use super::valid_range;

    #[test]
    fn test_inter_passes() {
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(valid_range, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_skip_static() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(valid_range, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_max_equals_min() {
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();

        for axis in &mut fvar.axis_instance_arrays.axes {
            if axis.axis_tag == fontations::write::types::Tag::new(b"wght") {
                axis.max_value = axis.min_value;
            }
        }

        let new_bytes = FontBuilder::new()
            .add_table(&fvar)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();
        testable.contents = new_bytes;

        let results = run_check(valid_range, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-range".to_string()),
        );
    }

    #[test]
    fn test_max_less_than_min() {
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();

        for axis in &mut fvar.axis_instance_arrays.axes {
            if axis.axis_tag == fontations::write::types::Tag::new(b"wght") {
                // Swap min and max to make max < min
                let temp = axis.min_value;
                axis.min_value = axis.max_value;
                axis.max_value = temp;
            }
        }

        let new_bytes = FontBuilder::new()
            .add_table(&fvar)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();
        testable.contents = new_bytes;

        let results = run_check(valid_range, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-range".to_string()),
        );
    }
}

use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/fvar/valid_range",
    title = "Validates fvar axis range and default values.",
    rationale = "
        Each axis defined in the fvar table must have:
        - A maximum value strictly greater than its minimum value. An axis
          where maxValue <= minValue is degenerate and defines no usable
          variation range, likely indicating a build error.
        - A default value within [minValue, maxValue]. A default outside
          the range indicates a malformed font.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/177",
    proposal = "https://github.com/fonttools/fontspector/issues/176"
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
        let default = axis.default_value();
        let max = axis.max_value();

        if max <= min {
            let message = format!(
                "The '{}' axis has maxValue ({}) <= minValue ({}). \
                 This defines no usable variation range.",
                tag, max, min
            );
            let mut status = Status::fail("invalid-range", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "fvar".to_string(),
                field_name: Some(format!("{} axis range", tag)),
                actual: Some(json!({"min": min, "max": max})),
                expected: Some(json!("maxValue > minValue")),
                message: message.clone(),
            });
            problems.push(status);
        }

        if default < min || default > max {
            let message = format!(
                "The default value ({}) for the '{}' axis is outside \
                 the range [{}, {}].",
                default, tag, min, max
            );
            let mut status = Status::fail("invalid-default", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "fvar".to_string(),
                field_name: Some(format!("{} axis default", tag)),
                actual: Some(json!(default)),
                expected: Some(json!({"min": min, "max": max})),
                message: message.clone(),
            });
            problems.push(status);
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

    #[test]
    fn test_default_below_min() {
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();

        for axis in &mut fvar.axis_instance_arrays.axes {
            if axis.axis_tag == fontations::write::types::Tag::new(b"wght") {
                axis.default_value = fontations::write::types::Fixed::from_f64(50.0);
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
            Some("invalid-default".to_string()),
        );
    }

    #[test]
    fn test_default_above_max() {
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();

        for axis in &mut fvar.axis_instance_arrays.axes {
            if axis.axis_tag == fontations::write::types::Tag::new(b"wght") {
                axis.default_value = fontations::write::types::Fixed::from_f64(1100.0);
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
            Some("invalid-default".to_string()),
        );
    }
}

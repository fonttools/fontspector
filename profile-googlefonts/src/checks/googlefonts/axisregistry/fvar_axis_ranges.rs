use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use google_fonts_axisregistry::AxisRegistry;

#[check(
    id = "googlefonts/axisregistry/fvar_axis_ranges",
    rationale = "
        Each axis range in the fvar table must be within the bounds defined in the
        Google Fonts Axis Registry, available at
        https://github.com/google/fonts/tree/main/axisregistry
    ",
    title = "Validate fvar axis ranges comply with Google Fonts Axis Registry bounds."
)]
fn fvar_axis_ranges(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not variable"
    );

    let registry = AxisRegistry::new();
    for axis in f.font().axes().iter() {
        let tag = axis.tag().to_string();
        if let Some(entry) = registry.get(&tag) {
            if axis.min_value() < entry.min_value() {
                problems.push(Status::fail(
                    "min-out-of-range",
                    &format!(
                        "The '{}' ({}) axis minimum value {} is below the Google Fonts Axis Registry minimum of {}.",
                        tag,
                        entry.display_name(),
                        axis.min_value(),
                        entry.min_value()
                    ),
                ));
            }
            if axis.max_value() > entry.max_value() {
                problems.push(Status::fail(
                    "max-out-of-range",
                    &format!(
                        "The '{}' ({}) axis maximum value {} exceeds the Google Fonts Axis Registry maximum of {}.",
                        tag,
                        entry.display_name(),
                        axis.max_value(),
                        entry.max_value()
                    ),
                ));
            }
        }
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    use fontations::skrifa::raw::TableProvider;
    use fontations::write::from_obj::ToOwnedTable;
    use fontations::write::tables::fvar::Fvar;
    use fontations::write::FontBuilder;
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, assert_skip, run_check, test_able,
    };
    use fontspector_checkapi::{FileTypeConvert, StatusCode, TTF};

    use super::fvar_axis_ranges;

    #[test]
    fn test_inter_passes() {
        // Inter has wght (100-900) and slnt (-10-0) axes, both within GF Axis Registry bounds
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(fvar_axis_ranges, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_skip_non_variable() {
        // Static fonts should be skipped
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(fvar_axis_ranges, testable);
        assert_skip(&results);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_wght_max_out_of_range() {
        // Modify Inter's wght axis max value to 1100 (GF Axis Registry max for wght is 1000)
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();

        // Get the fvar table and convert to owned/writable
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();

        // Find and modify the wght axis max value
        for axis in &mut fvar.axis_instance_arrays.axes {
            if axis.axis_tag == fontations::write::types::Tag::new(b"wght") {
                axis.max_value = fontations::write::types::Fixed::from_f64(1100.0);
            }
        }

        // Build new font with modified fvar
        let new_bytes = FontBuilder::new()
            .add_table(&fvar)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();

        testable.contents = new_bytes;

        let results = run_check(fvar_axis_ranges, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("max-out-of-range".to_string()),
        );
    }
}

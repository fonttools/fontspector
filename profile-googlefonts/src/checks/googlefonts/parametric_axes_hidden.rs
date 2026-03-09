use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use google_fonts_axisregistry::AxisRegistry;
use serde_json::json;

/// Known parametric axis tags from the Google Fonts axis registry.
/// These axes control fine-grained typographic parameters and should
/// be hidden from end users in most font selection UIs.
const PARAMETRIC_AXES: &[&str] = &[
    "XOPQ", // Thick Stroke
    "XTRA", // Counter Width
    "XTFI", // X transparent figures
    "YOPQ", // Thin Stroke
    "YTAS", // Ascender Height
    "YTDE", // Descender Depth
    "YTFI", // Figure Height
    "YTLC", // Lowercase Height
    "YTUC", // Uppercase Height
];

#[check(
    id = "googlefonts/parametric_axes_hidden",
    title = "Ensure parametric axes have the hidden flag set.",
    rationale = "
        Parametric axes (XOPQ, YOPQ, XTRA, YTAS, YTDE, YTFI, YTLC, YTUC, XTFI)
        are used for fine-grained control of typeface parameters and are not
        intended for direct use by end users. These axes should have the
        HIDDEN_AXIS flag set in the fvar table to prevent them from appearing
        in font selection UIs.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/651"
)]
fn parametric_axes_hidden(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font."
    );
    let registry = AxisRegistry::new();
    let mut problems = vec![];

    for axis in f.font().axes().iter() {
        let tag = axis.tag().to_string();
        if !PARAMETRIC_AXES.contains(&tag.as_str()) {
            continue;
        }
        if !axis.is_hidden() {
            let display_name = registry
                .get(&tag)
                .and_then(|e| e.display_name.as_deref())
                .unwrap_or(&tag);
            let message = format!(
                "Parametric axis '{}' ({}) does not have the HIDDEN_AXIS flag set. \
                 Parametric axes should be hidden from end users.",
                tag, display_name
            );
            let mut status = Status::fail("parametric-not-hidden", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "fvar".to_string(),
                field_name: Some(format!("axis '{}' flags", tag)),
                actual: Some(json!("HIDDEN_AXIS not set")),
                expected: Some(json!("HIDDEN_AXIS set")),
                message,
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

    use super::parametric_axes_hidden;

    #[test]
    fn test_skip_static_font() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(parametric_axes_hidden, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_pass_no_parametric_axes() {
        // Inter has wght and slnt axes, neither parametric
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(parametric_axes_hidden, testable);
        assert_pass(&results);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_fail_parametric_axis_not_hidden() {
        // Start from Inter (a variable font) and add an XOPQ parametric axis without HIDDEN flag
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();

        // Add a parametric axis (XOPQ) with flags=0 (not hidden)
        fvar.axis_instance_arrays
            .axes
            .push(fontations::write::tables::fvar::VariationAxisRecord {
                axis_tag: fontations::write::types::Tag::new(b"XOPQ"),
                min_value: fontations::write::types::Fixed::from_f64(10.0),
                default_value: fontations::write::types::Fixed::from_f64(88.0),
                max_value: fontations::write::types::Fixed::from_f64(200.0),
                flags: 0, // NOT hidden — should trigger fail
                axis_name_id: fontations::write::types::NameId::new(256),
            });

        // Add a third coordinate to each instance for the new axis
        for instance in &mut fvar.axis_instance_arrays.instances {
            instance
                .coordinates
                .push(fontations::write::types::Fixed::from_f64(88.0));
        }

        let new_bytes = FontBuilder::new()
            .add_table(&fvar)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();
        testable.contents = new_bytes;

        let results = run_check(parametric_axes_hidden, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("parametric-not-hidden".to_string()),
        );
    }
}

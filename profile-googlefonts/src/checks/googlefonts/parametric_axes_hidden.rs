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

    use fontspector_checkapi::codetesting::{assert_pass, assert_skip, run_check, test_able};

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
}

use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "linegaps",
    rationale = "
        The LineGap value is a space added to the line height created by the union
        of the (typo/hhea)Ascender and (typo/hhea)Descender. It is handled differently
        according to the environment.

        This leading value will be added above the text line in most desktop apps.
        It will be shared above and under in web browsers, and ignored in Windows
        if Use_Typo_Metrics is disabled.

        For better linespacing consistency across platforms,
        (typo/hhea)LineGap values must be 0.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4133",
    proposal = "https://googlefonts.github.io/gf-guide/metrics.html",
    title = "Checking Vertical Metric linegaps."
)]
fn linegaps(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);

    let os2 = f
        .font()
        .os2()
        .map_err(|_| FontspectorError::General("OS/2 table missing".to_string()))?;
    let hhea = f
        .font()
        .hhea()
        .map_err(|_| FontspectorError::General("hhea table missing".to_string()))?;
    let mut problems = vec![];
    if hhea.line_gap().to_i16() != 0 {
        let value = hhea.line_gap().to_i16();
        let message = "hhea lineGap is not equal to 0.";
        let mut status = Status::warn("hhea", message);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "hhea".to_string(),
            field_name: Some("lineGap".to_string()),
            actual: Some(json!(value)),
            expected: Some(json!(0)),
            message: message.to_string(),
        });
        problems.push(status);
    }
    if os2.s_typo_line_gap() != 0 {
        let value = os2.s_typo_line_gap();
        let message = "OS/2 sTypoLineGap is not equal to 0.";
        let mut status = Status::warn("OS/2", message);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("sTypoLineGap".to_string()),
            actual: Some(json!(value)),
            expected: Some(json!(0)),
            message: message.to_string(),
        });
        problems.push(status);
    }
    return_result(problems)
}

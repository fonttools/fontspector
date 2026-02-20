use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "os2_metrics_match_hhea",
    rationale = "
        OS/2 and hhea vertical metric values should match. This will produce the
        same linespacing on Mac, GNU+Linux and Windows.

        - Mac OS X uses the hhea values.
        - Windows uses OS/2 or Win, depending on the OS or fsSelection bit value.

        When OS/2 and hhea vertical metrics match, the same linespacing results on
        macOS, GNU+Linux and Windows. Note that fixing this issue in a previously
        released font may cause reflow in user documents and unhappy users.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Checking OS/2 Metrics match hhea Metrics."
)]
fn os2_metrics_match_hhea(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);

    skip!(
        f.is_cjk_font(Some(context)),
        "cjk-font",
        "Actually I'm not sure why we don't check this on CJK fonts."
    );

    let os2 = f
        .font()
        .os2()
        .map_err(|_| FontspectorError::General("OS/2 table missing".to_string()))?;
    let hhea = f
        .font()
        .hhea()
        .map_err(|_| FontspectorError::General("hhea table missing".to_string()))?;
    let mut problems = vec![];
    if os2.s_typo_ascender() != hhea.ascender().to_i16() {
        let os2_val = os2.s_typo_ascender();
        let hhea_val = hhea.ascender().to_i16();
        let message = format!(
            "OS/2 sTypoAscender ({}) and hhea ascent ({}) must be equal.",
            os2_val, hhea_val
        );
        let mut status = Status::fail("ascender", &message);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("sTypoAscender".to_string()),
            actual: Some(json!(os2_val)),
            expected: Some(json!(hhea_val)),
            message,
        });
        problems.push(status);
    }
    if os2.s_typo_descender() != hhea.descender().to_i16() {
        let os2_val = os2.s_typo_descender();
        let hhea_val = hhea.descender().to_i16();
        let message = format!(
            "OS/2 sTypoDescender ({}) and hhea descent ({}) must be equal.",
            os2_val, hhea_val
        );
        let mut status = Status::fail("descender", &message);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("sTypoDescender".to_string()),
            actual: Some(json!(os2_val)),
            expected: Some(json!(hhea_val)),
            message,
        });
        problems.push(status);
    }
    if os2.s_typo_line_gap() != hhea.line_gap().to_i16() {
        let os2_val = os2.s_typo_line_gap();
        let hhea_val = hhea.line_gap().to_i16();
        let message = format!(
            "OS/2 sTypoLineGap ({}) and hhea lineGap ({}) must be equal.",
            os2_val, hhea_val
        );
        let mut status = Status::fail("lineGap", &message);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("sTypoLineGap".to_string()),
            actual: Some(json!(os2_val)),
            expected: Some(json!(hhea_val)),
            message,
        });
        problems.push(status);
    }
    return_result(problems)
}

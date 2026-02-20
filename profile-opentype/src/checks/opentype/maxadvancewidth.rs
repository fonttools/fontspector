use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/maxadvancewidth",
    title = "MaxAdvanceWidth is consistent with values in the Hmtx and Hhea tables?",
    rationale = "
        The 'hhea' table contains a field which specifies the maximum advance width.
        This value should be consistent with the maximum advance width of all glyphs
        specified in the 'hmtx' table.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
)]
fn maxadvancewidth(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let hhea_advance_width_max = f.font().hhea()?.advance_width_max().to_u16();
    let hmtx_advance_width_max = f
        .font()
        .hmtx()?
        .h_metrics()
        .iter()
        .map(|m| m.advance.get())
        .max()
        .unwrap_or_default();
    let mut problems = vec![];
    if hmtx_advance_width_max != hhea_advance_width_max {
        let msg = format!(
            "AdvanceWidthMax mismatch: expected {hmtx_advance_width_max} from hmtx; got {hhea_advance_width_max} for hhea"
        );
        let mut status = Status::fail("mismatch", &msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "hhea".to_string(),
            field_name: Some("advanceWidthMax".to_string()),
            actual: Some(json!(hhea_advance_width_max)),
            expected: Some(json!(hmtx_advance_width_max)),
            message: msg,
        });
        problems.push(status);
    }
    return_result(problems)
}

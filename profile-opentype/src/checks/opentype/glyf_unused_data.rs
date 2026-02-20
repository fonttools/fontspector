use fontations::skrifa::{raw::TableProvider, Tag};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;
use std::cmp::Ordering;

#[check(
    id="opentype/glyf_unused_data",
    rationale="
        This check validates the structural integrity of the glyf table,
        by checking that all glyphs referenced in the loca table are
        actually present in the glyf table and that there is no unused
        data at the end of the glyf table. A failure here indicates a
        problem with the font compiler.
    ",
    proposal="https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    title="Is there any unused data at the end of the glyf table?"
)]
fn glyf_unused_data(t: &Testable, _context: &Context) -> CheckFnResult {
    let ttf = testfont!(t);
    let glyf = ttf
        .font()
        .table_data(Tag::new(b"glyf"))
        .ok_or(FontspectorError::skip("no-glyf", "No glyf table"))?;
    let loca = ttf
        .font()
        .loca(None)
        .map_err(|_| FontspectorError::General("No loca table".to_string()))?;
    let mut problems = vec![];
    if let Some(last_index) = loca.get_raw(loca.len()) {
        match glyf.len().cmp(&(last_index as usize)) {
            Ordering::Greater => {
                let msg = "Unused data at the end of the glyf table";
                let mut status = Status::fail("unreachable-data", msg);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "glyf".to_string(),
                    field_name: None,
                    actual: Some(json!(glyf.len())),
                    expected: Some(json!(last_index as usize)),
                    message: msg.to_string(),
                });
                problems.push(status);
            }
            Ordering::Less => {
                let msg = "Missing data at the end of the glyf table";
                let mut status = Status::fail("missing-data", msg);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "glyf".to_string(),
                    field_name: None,
                    actual: Some(json!(glyf.len())),
                    expected: Some(json!(last_index as usize)),
                    message: msg.to_string(),
                });
                problems.push(status);
            }
            Ordering::Equal => {
                // Pass
            }
        }
        return_result(problems)
    } else {
        Err(FontspectorError::General("Invalid loca table".to_string()))
    }
}

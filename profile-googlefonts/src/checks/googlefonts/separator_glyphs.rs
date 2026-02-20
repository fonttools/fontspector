use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

const SEPARATOR_GLYPHS: [u32; 2] = [
    0x2028, // LINE SEPARATOR
    0x2029, // PARAGRAPH SEPARATOR
];

#[check(
    id = "googlefonts/separator_glyphs",
    rationale = "
        U+2028 and U+2029 should be present; otherwise tofu is displayed.
        (whitespace_ink will check that they are empty)
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/93",
    title = "Font has correct separator glyphs?"
)]
fn separator_glyphs(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let codepoints = f.codepoints(Some(context));
    let mut problems = vec![];
    let missing = SEPARATOR_GLYPHS
        .iter()
        .filter(|&&cp| !codepoints.contains(&cp))
        .collect::<Vec<_>>();
    if missing.is_empty() {
        problems.push(Status::pass());
    } else {
        for &cp in missing.iter() {
            let msg = format!("Missing separator glyph U+{cp:X}");
            let mut status = Status::warn("missing-separator-glyphs", &msg);
            status.add_metadata(Metadata::FontProblem {
                message: msg.clone(),
                context: Some(json!({
                    "codepoint": format!("U+{cp:X}"),
                    "codepoint_decimal": cp
                })),
            });
            problems.push(status);
        }
    }
    return_result(problems)
}

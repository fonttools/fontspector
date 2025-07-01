use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

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
    let missing = SEPARATOR_GLYPHS
        .iter()
        .filter(|&&cp| !codepoints.contains(&cp))
        .collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(Status::just_one_pass())
    } else {
        Ok(Status::just_one_warn(
            "missing-separator-glyphs",
            &format!(
                "The following separator glyphs are missing:\n{}",
                bullet_list(context, missing.iter().map(|&cp| format!("U+{cp:X}")))
            ),
        ))
    }
}

use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "fontwerk/soft_hyphen",
    rationale = "
        The 'Soft Hyphen' character (codepoint 0x00AD) is not used in Fontwerk fonts.
        Based on custom projects, where we faced issues with the soft-hyphen we decided to not include it in our fonts.
        We are aware of the unicode recommendations regarding the soft-hyphen:
        https://www.unicode.org/reports/tr14/#SoftHyphen

        But even a soft-hyphen with a width of 0 and no outline causes issues in some applications, therefore it is not recommended.
    ",
    proposal = "https://github.com/ollimeier/fontspector/issues/6",
    title = "Does the font contain a soft hyphen?"
)]
fn soft_hyphen(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    Ok(if f.codepoints(Some(context)).contains(&0x00AD) {
        Status::just_one_fail(
            "softhyphen",
            "This font has a 'Soft Hyphen' character. Please remove it.",
        )
    } else {
        Status::just_one_pass()
    })
}

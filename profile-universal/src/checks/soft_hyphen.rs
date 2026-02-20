use fontations::{skrifa::MetadataProvider, types::GlyphId};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "soft_hyphen",
    rationale = "
        The 'Soft Hyphen' character (codepoint 0x00AD) is used to mark
        a hyphenation possibility within a word in the absence of or
        overriding dictionary hyphenation.

        It is sometimes designed empty with no width (such as a control character),
        sometimes the same as the traditional hyphen, sometimes double encoded with
        the hyphen.

        That being said, it is recommended to not include it in the font at all,
        because discretionary hyphenation should be handled at the level of the
        shaping engine, not the font. Also, even if present, the software would
        not display that character.

        More discussion at:
        https://typedrawers.com/discussion/2046/special-dash-things-softhyphen-horizontalbar
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4046",
    proposal = "https://github.com/fonttools/fontbakery/issues/3486",
    title = "Does the font contain a soft hyphen?"
)]
fn soft_hyphen(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    if f.codepoints(Some(context)).contains(&0x00AD) {
        let glyphid = f.font().charmap().map(0xad_u32).unwrap_or(GlyphId::new(0));
        let glyphname = f.glyph_name_for_id_synthesise(glyphid);

        let mut status = Status::warn("softhyphen", "This font has a 'Soft Hyphen' character.");
        status.add_metadata(
            Metadata::GlyphProblem {
                glyph_name: glyphname.clone(),
                glyph_id: glyphid.to_u32(),
                userspace_location: None,
                position: None,
                actual: Some(json!({ "codepoint": "U+00AD", })),
                expected: None,
                message: "The 'Soft Hyphen' character is used to mark a hyphenation possibility within a word, but it is recommended to not include it in the font at all, because discretionary hyphenation should be handled at the level of the shaping engine, not the font. Also, even if present, the software would not display that character.".to_string(),
            }
        );
        problems.push(status);
    }
    return_result(problems)
}
// def check_soft_hyphen(ttFont):
//     """Does the font contain a soft hyphen?"""
//     if 0x00AD in ttFont["cmap"].getBestCmap().keys():
//         yield WARN, Message("softhyphen", "This font has a 'Soft Hyphen' character.")
//     else:
//         yield PASS, "Looks good!"

use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{
    pens::HasInkPen, prelude::*, testfont, FileTypeConvert, DEFAULT_LOCATION,
};
use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};

const EXTRA_NON_DRAWING: [u32; 6] = [0x180E, 0x200B, 0x2028, 0x2029, 0x2060, 0xFEFF];
const BUT_NOT: [u32; 2] = [0xAD, 0x1680];

#[check(
    id = "whitespace_ink",
    rationale = "
           This check ensures that certain whitespace glyphs are empty.
           Certain text layout engines will assume that these glyphs are empty,
           and will not draw them; if they were in fact not designed to be
           empty, the result will be text layout that is not as expected.
       ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    proposal = "https://github.com/fonttools/fontspector/issues/93",
    title = "Whitespace glyphs have ink?"
)]
fn whitespace_ink(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let inky = f
        .codepoints(Some(context))
        .into_iter()
        .filter(|cp| {
            (EXTRA_NON_DRAWING.contains(cp)
                || (char::from_u32(*cp)
                    .map(|c| {
                        matches!(
                            c.general_category(),
                            GeneralCategory::SpaceSeparator | GeneralCategory::Format
                        )
                    })
                    .unwrap_or(false)))
                && !BUT_NOT.contains(cp)
        })
        .map(|cp| {
            #[allow(clippy::unwrap_used)]
            let gid = f.font().charmap().map(cp).unwrap();
            (cp, gid)
        })
        .filter(|(_cp, gid)| {
            let mut has_ink_pen = HasInkPen::new();
            f.draw_glyph(*gid, &mut has_ink_pen, DEFAULT_LOCATION)
                .ok()
                .and(has_ink_pen.has_ink().then_some(()))
                .is_some()
        })
        .map(|(_cp, gid)| f.glyph_name_for_id_synthesise(gid))
        .collect::<Vec<_>>();
    if inky.is_empty() {
        Ok(Status::just_one_pass())
    } else {
        Ok(Status::just_one_fail(
            "has-ink",
            &format!(
                "The following glyphs have ink; they should be replaced by an empty glyph:\n{}",
                bullet_list(context, inky)
            ),
        ))
    }
}

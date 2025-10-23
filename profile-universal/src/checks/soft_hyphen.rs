use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Source, SourceFile};

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
    title = "Does the font contain a soft hyphen?",
    fix_source = sourcefix_softhyphen,
)]
fn soft_hyphen(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    Ok(if f.codepoints(Some(context)).contains(&0x00AD) {
        Status::just_one_warn("softhyphen", "This font has a 'Soft Hyphen' character.")
    } else {
        Status::just_one_pass()
    })
}

fn sourcefix_softhyphen(s: &mut SourceFile) -> FixFnResult {
    fn fix_a_ufo(font: &mut norad::Font) -> FixFnResult {
        let soft_hyphen_glyph = font
            .default_layer()
            .iter()
            .find(|g| g.codepoints.contains(0xAD as char))
            .map(|g| g.name().to_string());
        if let Some(name) = soft_hyphen_glyph {
            log::info!("Removing soft hyphen glyph '{name}' from UFO file.");
            font.default_layer_mut().remove_glyph(&name);
            Ok(true)
        } else {
            log::info!("No soft hyphen glyph found in UFO file.");
            Ok(false)
        }
    }
    match s.source {
        Source::Ufo(ref mut font) => fix_a_ufo(font),
        Source::Designspace(ref mut ds) => ds.apply_fix(&fix_a_ufo),
        Source::Glyphs(ref mut font) => match &mut **font {
            glyphslib::Font::Glyphs2(glyphs2) => {
                let had_one = glyphs2.glyphs.iter().any(|g| g.unicode.contains(&0xAD));
                if !had_one {
                    log::info!("No soft hyphen glyph found in Glyphs file.");
                    return Ok(false);
                }
                glyphs2.glyphs.retain(|g| !g.unicode.contains(&0xAD));
                log::info!("Removing soft hyphen glyph from Glyphs file.");
                Ok(true)
            }
            glyphslib::Font::Glyphs3(glyphs3) => {
                let had_one = glyphs3.glyphs.iter().any(|g| g.unicode.contains(&0xAD));
                if !had_one {
                    log::info!("No soft hyphen glyph found in Glyphs file.");
                    return Ok(false);
                }
                glyphs3.glyphs.retain(|g| !g.unicode.contains(&0xAD));
                log::info!("Removing soft hyphen glyph from Glyphs file.");
                Ok(true)
            }
        },
    }
}

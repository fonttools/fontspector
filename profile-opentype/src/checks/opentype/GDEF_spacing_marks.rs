use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "opentype/GDEF_spacing_marks",
    rationale = "
        Glyphs in the GDEF mark glyph class should be non-spacing.

        Spacing glyphs in the GDEF mark glyph class may have incorrect anchor
        positioning that was only intended for building composite glyphs during design.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2877",
    title = "Check glyphs in mark glyph class are non-spacing."
)]
fn GDEF_spacing_marks(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let hmtx = font.font().hmtx()?;
    let gdef = font
        .font()
        .gdef()
        .map_err(|_| FontspectorError::skip("no-gdef", "GDEF table unreadable or not present"))?;
    let glyph_classdef = gdef.glyph_class_def().ok_or_else(|| {
        FontspectorError::skip("no-glyph-class-def", "GDEF table has no GlyphClassDef")
    })??;
    let nonspacing_mark_glyphs = bullet_list(
        context,
        glyph_classdef
            .iter()
            .filter(|(glyph, class)| *class == 3 && hmtx.advance((*glyph).into()).unwrap_or(0) > 0)
            .map(|(glyph, _)| font.glyph_name_for_id_synthesise(glyph)),
    );
    if !nonspacing_mark_glyphs.is_empty() {
        return Ok(Status::just_one_warn("spacing-mark-glyphs", &format!(
            "The following glyphs seem to be spacing (because they have width > 0 on the hmtx table) so they may be in the GDEF mark glyph class by mistake, or they should have zero width instead:\n\n{nonspacing_mark_glyphs}"
        )));
    }

    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{assert_pass, assert_skip, run_check, test_able};

    #[test]
    fn test_gdef_spacing_marks_skip_no_gdef() {
        let mut testable = test_able("familysans/FamilySans-Regular.ttf");
        fontspector_checkapi::codetesting::remove_table(&mut testable, b"GDEF");
        let result = run_check(super::GDEF_spacing_marks, testable);
        assert_skip(&result);
    }

    #[test]
    fn test_gdef_spacing_marks_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let result = run_check(super::GDEF_spacing_marks, testable);
        assert_pass(&result);
    }
}

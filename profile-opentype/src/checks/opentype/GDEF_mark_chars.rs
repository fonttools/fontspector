use fontations::skrifa::{raw::tables::gdef::GlyphClassDef, MetadataProvider};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};

pub(crate) fn is_nonspacing_mark(c: char) -> bool {
    matches!(
        c.general_category(),
        GeneralCategory::NonspacingMark | GeneralCategory::EnclosingMark
    )
}

#[check(
    id = "opentype/GDEF_mark_chars",
    rationale = "Mark characters should be in the GDEF mark glyph class.",
    proposal = "https://github.com/fonttools/fontbakery/issues/2877",
    title = "Check mark characters are in GDEF mark glyph class."
)]
fn GDEF_mark_chars(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    skip!(
        !font.has_table(b"GDEF"),
        "no-gdef",
        "GDEF table not present"
    );
    let mark_chars_not_in_gdef_mark = bullet_list(
        context,
        font.font()
            .charmap()
            .mappings()
            .filter(|(u, gid)| {
                char::from_u32(*u).is_some_and(is_nonspacing_mark)
                    && font.gdef_class(*gid) != GlyphClassDef::Mark
            })
            .map(|(u, gid)| {
                let name = font.glyph_name_for_id_synthesise(gid);
                format!("U+{u:04X} ({name})")
            }),
    );
    if !mark_chars_not_in_gdef_mark.is_empty() {
        return Ok(Status::just_one_warn(
            "mark-chars",
            &format!(
                "The following mark characters should be in the GDEF mark glyph class:\n\n{mark_chars_not_in_gdef_mark}"
            ),
        ));
    }

    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{assert_pass, assert_skip, run_check, test_able};

    #[test]
    fn test_gdef_mark_chars_skip_no_gdef() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        fontspector_checkapi::codetesting::remove_table(&mut testable, b"GDEF");
        let result = run_check(super::GDEF_mark_chars, testable);
        assert_skip(&result);
    }

    #[test]
    fn test_gdef_mark_chars_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let result = run_check(super::GDEF_mark_chars, testable);
        assert_pass(&result);
    }
}

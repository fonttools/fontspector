use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::{raw::tables::gdef::GlyphClassDef, MetadataProvider};
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
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{
        assert_messages_contain, assert_pass, assert_results_contain, assert_skip, remove_table,
        run_check, set_gdef_glyph_classes, test_able,
    };
    use fontspector_checkapi::{FileTypeConvert, StatusCode, TestFont, Testable, TTF};
    use skrifa::MetadataProvider;
    use std::collections::BTreeSet;

    /// All glyph names in the font which map from a nonspacing/enclosing mark.
    fn mark_glyph_names(t: &Testable) -> Vec<String> {
        let f: TestFont = TTF.from_testable(t).unwrap();
        f.font()
            .charmap()
            .mappings()
            .filter_map(|(cp, gid)| char::from_u32(cp).map(|c| (c, gid)))
            .filter(|(c, _)| super::is_nonspacing_mark(*c))
            .map(|(_, gid)| f.glyph_name_for_id_synthesise(gid))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    #[test]
    fn test_gdef_mark_chars_skip_no_gdef() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        remove_table(&mut testable, b"GDEF");
        let result = run_check(super::GDEF_mark_chars, testable);
        assert_skip(&result);
    }

    #[test]
    fn test_gdef_mark_chars_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let result = run_check(super::GDEF_mark_chars, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_gdef_mark_chars_empty_gdef_warn() {
        // A GDEF table with no mark glyphs declared should warn about
        // every mark character, including U+0301 (acutecomb).
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        set_gdef_glyph_classes(&mut testable, &[]).unwrap();
        let result = run_check(super::GDEF_mark_chars, testable);
        assert_results_contain(&result, StatusCode::Warn, Some("mark-chars".to_string()));
        assert_messages_contain(&result, "U+0301");
    }

    #[test]
    fn test_gdef_mark_chars_properly_declared_pass() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        let names = mark_glyph_names(&testable);
        let classes: Vec<(&str, u16)> = names.iter().map(|n| (n.as_str(), 3)).collect();
        set_gdef_glyph_classes(&mut testable, &classes).unwrap();
        let result = run_check(super::GDEF_mark_chars, testable);
        assert_pass(&result);
    }
}

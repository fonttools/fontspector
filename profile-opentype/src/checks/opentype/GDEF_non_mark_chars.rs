use crate::checks::opentype::GDEF_mark_chars::is_nonspacing_mark;
use fontations::skrifa::{raw::TableProvider, GlyphId16, MetadataProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};

fn swaption<T, U>(a: T, b: Option<U>) -> Option<(T, U)> {
    b.map(|b| (a, b))
}

#[check(
    id = "opentype/GDEF_non_mark_chars",
    rationale = "
        Glyphs in the GDEF mark glyph class become non-spacing and may be repositioned
        if they have mark anchors.

        Only combining mark glyphs should be in that class. Any non-mark glyph
        must not be in that class, in particular spacing glyphs.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2877",
    title = "Check GDEF mark glyph class doesn't have characters that are not marks."
)]
fn GDEF_non_mark_chars(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let gdef = f
        .font()
        .gdef()
        .map_err(|_| FontspectorError::skip("no-gdef", "GDEF table unreadable or not present"))?;
    let glyph_classdef = gdef.glyph_class_def().ok_or_else(|| {
        FontspectorError::skip("no-glyph-class-def", "GDEF table has no GlyphClassDef")
    })??;
    let codepoints = f.codepoints(Some(context));
    let non_mark_gids = codepoints
        .iter()
        .flat_map(|cp| char::from_u32(*cp))
        .filter(|&cp| !is_nonspacing_mark(cp))
        .flat_map(|cp| swaption(cp, f.font().charmap().map(cp)))
        .flat_map(|(cp, gid)| swaption(cp, GlyphId16::try_from(gid).ok()));
    let mut non_mark_gids_in_mark: Vec<_> = non_mark_gids
        .filter(|(_cp, gid)| glyph_classdef.get(*gid) == 3)
        .collect();
    // Sort by GID
    non_mark_gids_in_mark.sort_by_key(|(_cp, gid)| *gid);
    if !non_mark_gids_in_mark.clone().is_empty() {
        // Create a Metadata item for each glyph
        let metadata = non_mark_gids_in_mark
            .iter()
            .map(|&(cp, gid)| Metadata::GlyphProblem {
                glyph_id: gid.into(),
                glyph_name: f.glyph_name_for_id_synthesise(gid),
                message: format!(
                    "U+{:04X} ({}) is not a non-spacing mark but is in the GDEF mark glyph class",
                    cp as u32,
                    f.glyph_name_for_id_synthesise(gid)
                ),
                userspace_location: None,
                position: None,
                actual: None,
                expected: None,
            });
        let mut warn = Status::warn(
            "non-mark-chars",
            &format!(
                "The following non-mark characters should not be in the GDEF mark glyph class:\n\n{}",
                bullet_list(
                    context,
                    non_mark_gids_in_mark.iter().map(|(cp, gid)| format!(
                        "U+{:04X} ({})",
                        *cp as u32,
                        f.glyph_name_for_id_synthesise(*gid)
                    ))
                ),
            ),
        );
        warn.metadata.extend(metadata);
        return return_result(vec![warn]);
    }
    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{assert_pass, assert_skip, run_check, test_able};

    #[test]
    fn test_gdef_non_mark_chars_skip_no_gdef() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        fontspector_checkapi::codetesting::remove_table(&mut testable, b"GDEF");
        let result = run_check(super::GDEF_non_mark_chars, testable);
        assert_skip(&result);
    }

    #[test]
    fn test_gdef_non_mark_chars_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let result = run_check(super::GDEF_non_mark_chars, testable);
        assert_pass(&result);
    }
}

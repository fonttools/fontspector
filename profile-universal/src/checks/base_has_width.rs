use std::collections::HashMap;

use fontations::skrifa::{
    raw::{tables::gdef::GlyphClassDef, TableProvider},
    GlyphId, MetadataProvider,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;
use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};

fn is_space(c: char) -> bool {
    matches!(
        c.general_category(),
        GeneralCategory::SpaceSeparator
            | GeneralCategory::LineSeparator
            | GeneralCategory::ParagraphSeparator
            | GeneralCategory::Format
            | GeneralCategory::NonspacingMark
            | GeneralCategory::Control
    )
}

/// Unicode codepoints that are expected to have zero advance width.
const ZERO_WIDTH_CHARS: &[(u32, &str)] = &[
    (0xFEFF, "ZERO WIDTH NO-BREAK SPACE"),
    (0x200B, "ZERO WIDTH SPACE"),
    (0x200C, "ZERO WIDTH NON-JOINER"),
    (0x200D, "ZERO WIDTH JOINER"),
    (0x2060, "WORD JOINER"),
    (0xFFFE, "<noncharacter-FFFE>"),
];

fn is_expected_zero_width(codepoint: u32) -> bool {
    ZERO_WIDTH_CHARS.iter().any(|(cp, _)| *cp == codepoint)
}

#[check(
    id = "base_has_width",
    rationale = "
        Base characters should have non-zero advance width.
        Additionally, certain Unicode characters (such as zero-width joiners
        and zero-width spaces) are expected to have zero advance width.
        If they have a non-zero width, they may cause unexpected spacing
        in text layout.
    ",
    proposal = "Rod on chat",
    proposal = "https://github.com/fonttools/fontspector/issues/518",
    title = "Check base characters have non-zero advance width."
)]
fn base_has_width(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let hmtx = font.font().hmtx()?;
    let charmap = font.font().charmap();
    let mut problems = vec![];
    let reverse_charmap: HashMap<_, _> = font
        .font()
        .charmap()
        .mappings()
        .map(|(c, g)| (g, c))
        .collect();

    // Check 1: base glyphs should not have zero advance width
    let mut zero_width_bases = vec![];
    for (gid, metric) in hmtx.h_metrics().iter().enumerate() {
        let gid = GlyphId::new(gid as u32);
        if metric.advance() == 0 && font.gdef_class(gid) != GlyphClassDef::Mark {
            let codepoint = reverse_charmap.get(&gid);
            if codepoint == Some(&0) || codepoint.is_none() {
                continue;
            }
            // Skip characters that are expected to have zero width
            if codepoint.is_some_and(|cp| is_expected_zero_width(*cp)) {
                continue;
            }
            if codepoint
                .and_then(|c| char::from_u32(*c))
                .is_some_and(is_space)
            {
                continue;
            }
            #[allow(clippy::unwrap_used)]
            let name = font.glyph_name_for_id_synthesise(gid);
            if name == "NULL" {
                continue;
            }
            zero_width_bases.push(format!("{name} ({codepoint:?})"));
        }
    }
    if !zero_width_bases.is_empty() {
        problems.push(Status::fail(
            "zero-width-bases",
            &format!(
                "The following glyphs had zero advance width:\n\n{}",
                bullet_list(context, zero_width_bases),
            ),
        ));
    }

    // Check 2: zero-width characters should have zero advance width
    for &(codepoint, name) in ZERO_WIDTH_CHARS {
        if let Some(gid) = charmap.map(codepoint) {
            let advance = hmtx.advance(gid).unwrap_or(0);
            if advance != 0 {
                let glyph_name = font.glyph_name_for_id_synthesise(gid);
                let message =
                    format!("U+{codepoint:04X} {name} has non-zero advance width: {advance}");
                let mut status = Status::warn("non-zero-advance", &message);
                status.add_metadata(Metadata::GlyphProblem {
                    glyph_name,
                    glyph_id: gid.to_u32(),
                    userspace_location: None,
                    position: None,
                    actual: Some(json!({ "advance_width": advance })),
                    expected: Some(json!({ "advance_width": 0 })),
                    message,
                });
                problems.push(status);
            }
        }
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, remap_glyph, run_check, test_able},
        StatusCode,
    };

    use super::base_has_width;

    #[test]
    fn test_base_has_width_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let results = run_check(base_has_width, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_zero_width_char_with_nonzero_advance() {
        // Map U+200B (ZERO WIDTH SPACE) to the "a" glyph which has a non-zero advance width.
        // This should trigger the "non-zero-advance" warning.
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        remap_glyph(&mut testable, 0x200B, "a").expect("remap failed");
        let results = run_check(base_has_width, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("non-zero-advance".to_string()),
        );
    }
}

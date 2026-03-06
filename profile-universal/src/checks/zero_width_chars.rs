use fontations::skrifa::{raw::TableProvider, MetadataProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

/// Unicode codepoints that should have zero advance width.
const ZERO_WIDTH_CHARS: &[(u32, &str)] = &[
    (0xFEFF, "ZERO WIDTH NO-BREAK SPACE"),
    (0x200B, "ZERO WIDTH SPACE"),
    (0x200C, "ZERO WIDTH NON-JOINER"),
    (0x200D, "ZERO WIDTH JOINER"),
    (0x2060, "WORD JOINER"),
    (0xFFFE, "<noncharacter-FFFE>"),
];

#[check(
    id = "zero_width_chars",
    rationale = "
        Certain Unicode characters are expected to have zero advance width.
        If they have a non-zero width, they may cause unexpected spacing
        in text layout.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/518",
    title = "Check that zero-width characters have zero advance width."
)]
fn zero_width_chars(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let charmap = f.font().charmap();
    let hmtx = f.font().hmtx()?;
    let mut problems = vec![];

    for &(codepoint, name) in ZERO_WIDTH_CHARS {
        if let Some(gid) = charmap.map(codepoint) {
            let advance = hmtx.advance(gid).unwrap_or(0);
            if advance != 0 {
                problems.push(Status::warn(
                    "non-zero-advance",
                    &format!("U+{codepoint:04X} {name} has non-zero advance width: {advance}"),
                ));
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use super::ZERO_WIDTH_CHARS;

    #[test]
    fn test_zero_width_chars_list() {
        // Verify all entries have valid Unicode codepoints
        for &(cp, name) in ZERO_WIDTH_CHARS {
            assert!(
                char::from_u32(cp).is_some() || cp == 0xFFFE,
                "Invalid codepoint U+{cp:04X} ({name})"
            );
        }
    }
}

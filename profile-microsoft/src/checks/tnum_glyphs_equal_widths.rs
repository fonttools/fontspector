use std::collections::HashMap;

use fontations::skrifa::GlyphId;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use rustybuzz::{ttf_parser, Face, UnicodeBuffer};

fn verify_widths(face: &Face, text: &str) -> HashMap<i32, Vec<GlyphId>> {
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);
    let features = vec![rustybuzz::Feature::new(
        ttf_parser::Tag::from_bytes_lossy(b"tnum"),
        1_u32,
        ..,
    )];
    buffer.guess_segment_properties();
    let glyph_buffer = rustybuzz::shape(face, &features, buffer);
    glyph_buffer
        .glyph_infos()
        .iter()
        .zip(glyph_buffer.glyph_positions())
        .map(|(info, pos)| {
            let width = pos.x_advance;
            (GlyphId::new(info.glyph_id), width)
        })
        .fold(HashMap::new(), |mut acc, (glyph_id, width)| {
            acc.entry(width).or_default().push(glyph_id);
            acc
        })
}

#[check(
    id = "tnum_glyphs_equal_widths",
    rationale = "
        Check to make sure all the tnum glyphs are the same width.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Widths of tabular number glyphs."
)]
fn tnum_glyphs_equal_widths(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];

    let mut face = Face::from_slice(&t.contents, 0).ok_or(FontspectorError::General(
        "Failed to load font file".to_string(),
    ))?;
    let check_text = context
        .configuration
        .get("TEST_STR")
        .and_then(|s| s.as_str())
        .unwrap_or("0123456789");
    skip!(
        !f.has_feature(true, "tnum"),
        "no-tnum",
        "Font does not contain tnum feature"
    );

    let format_glyphs_by_width = |glyphs_with_width: &HashMap<i32, Vec<GlyphId>>| {
        glyphs_with_width
            .iter()
            .map(|(width, glyphs)| {
                format!(
                    "Width {}: {}",
                    width,
                    glyphs
                        .iter()
                        .map(|gid| f.glyph_name_for_id_synthesise(*gid))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let variations_to_test: Vec<Vec<rustybuzz::Variation>> = if f.is_variable_font() {
        f.named_instances()
            .map(|(_name, coordinates)| {
                coordinates
                    .iter()
                    .map(|(tag, value)| rustybuzz::Variation {
                        tag: ttf_parser::Tag::from_bytes_lossy(tag.as_bytes()),
                        value: *value,
                    })
                    .collect()
            })
            .collect()
    } else {
        vec![vec![]]
    };

    for variation in variations_to_test {
        face.set_variations(&variation);
        let glyphs_with_width = verify_widths(&face, check_text);
        if glyphs_with_width.len() > 1 {
            problems.push(Status::fail(
                "tnum_glyphs_equal_widths",
                &format!(
                    "tnum glyphs appear not to align:\n{}",
                    format_glyphs_by_width(&glyphs_with_width)
                ),
            ));
        }
    }
    return_result(problems)
}

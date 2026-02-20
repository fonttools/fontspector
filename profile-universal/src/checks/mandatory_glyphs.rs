use fontations::skrifa::{GlyphId, MetadataProvider};
use fontspector_checkapi::{
    pens::HasInkPen, prelude::*, testfont, FileTypeConvert, Metadata, DEFAULT_LOCATION,
};
use serde_json::json;

#[check(
    id="mandatory_glyphs",
    rationale="
        The OpenType specification v1.8.2 recommends that the first glyph is the
        '.notdef' glyph without a codepoint assigned and with a drawing:

        The .notdef glyph is very important for providing the user feedback
        that a glyph is not found in the font. This glyph should not be left
        without an outline as the user will only see what looks like a space
        if a glyph is missing and not be aware of the active font’s limitation.

        https://docs.microsoft.com/en-us/typography/opentype/spec/recom#glyph-0-the-notdef-glyph

        Pre-v1.8, it was recommended that fonts should also contain 'space', 'CR'
        and '.null' glyphs. This might have been relevant for MacOS 9 applications.
    ",
    title="Font contains '.notdef' as its first glyph?",
    proposal="https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
)]
fn mandatory_glyphs(f: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let mut problems = vec![];
    let gid_0 = GlyphId::new(0);
    let notdef_name = font
        .glyph_name_for_id(gid_0)
        .unwrap_or_else(|| ".notdef".to_string());
    if font.glyph_name_for_id(gid_0) != Some(".notdef".to_string()) {
        // Is notdef somewhere else?!
        let notdef_gid = font
            .all_glyphs()
            .find(|g| font.glyph_name_for_id(*g) == Some(".notdef".to_string()));
        if let Some(notdef_gid) = notdef_gid {
            let message = "The '.notdef' glyph should be the font's first glyph.";
            let mut status = Status::warn("notdef-not-first", message);
            status.add_metadata(Metadata::GlyphProblem {
                glyph_name: ".notdef".to_string(),
                glyph_id: notdef_gid.to_u32(),
                userspace_location: None,
                position: None,
                actual: Some(json!({ "glyph_index": notdef_gid.to_u32() })),
                expected: Some(json!({ "glyph_index": 0 })),
                message: message.to_string(),
            });
            problems.push(status);
        } else {
            problems.push(Status::warn(
                "notdef-not-found",
                "Font should contain the '.notdef' glyph.",
            ))
        }
    }
    if let Some(cp) = font.font().charmap().mappings().find(|m| m.1 == gid_0) {
        let message = format!(
            "The '.notdef' glyph should not have a Unicode codepoint value assigned, but has 0x{:04X}.",
            cp.0
        );
        let mut status = Status::warn("notdef-has-codepoint", &message);
        status.add_metadata(Metadata::GlyphProblem {
            glyph_name: notdef_name.clone(),
            glyph_id: gid_0.to_u32(),
            userspace_location: None,
            position: None,
            actual: Some(json!({ "codepoint": cp.0 })),
            expected: Some(json!(null)),
            message,
        });
        problems.push(status);
    }
    let mut pen = HasInkPen::new();
    font.draw_glyph(gid_0, &mut pen, DEFAULT_LOCATION)?;
    if !pen.has_ink() {
        let message = "The '.notdef' glyph should contain a drawing, but it is blank.";
        let mut status = Status::fail("notdef-is-blank", message);
        status.add_metadata(Metadata::GlyphProblem {
            glyph_name: notdef_name,
            glyph_id: gid_0.to_u32(),
            userspace_location: None,
            position: None,
            actual: Some(json!({ "has_ink": false })),
            expected: Some(json!({ "has_ink": true })),
            message: message.to_string(),
        });
        problems.push(status);
    }

    return_result(problems)
}

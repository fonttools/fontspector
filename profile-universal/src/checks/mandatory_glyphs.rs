use fontspector_checkapi::{
    pens::ContourCountPen, prelude::*, testfont, FileTypeConvert, Metadata, SourceFile,
    DEFAULT_LOCATION,
};
use serde_json::json;
use skrifa::{raw::TableProvider, GlyphId, MetadataProvider};

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

        Specific requirements to the .notdef glyph:

        - Is glyph 0 named .notdef?
        - Does .notdef have a non-zero advance width?
        - Does glyph 0 (.notdef) have a non-empty outline?

        Pre-v1.8, it was recommended that fonts should also contain 'space', 'CR'
        and '.null' glyphs. This might have been relevant for MacOS 9 applications.
    ",
    title="Font contains '.notdef' as its first glyph?",
    proposal="https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    fix_source=sourcefix_mandatory_glyphs
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
    let mut pen = ContourCountPen::new();
    font.draw_glyph(gid_0, &mut pen, DEFAULT_LOCATION)?;
    let contour_count = pen.contour_count();
    if contour_count == 0 {
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
    } else if contour_count == 1 || contour_count == 2 {
        // .notdef has 1 or 2 contours. It looks like it is a TOFU.
        // passing information about the 2-contour TOFU
        problems.push(Status::pass());
    } else if contour_count == 3 {
        problems.push(Status::info(
            "notdef-contours-3",
            ".notdef has 3 contours. It might be a black box with a white question mark.",
        ));
    } else if contour_count == 4 {
        problems.push(Status::info(
            "notdef-contours-4",
            ".notdef has 4 contours. It might be a TOFU with a question mark.",
        ));
    } else if contour_count == 5 {
        problems.push(Status::info(
            "notdef-contours-5",
            ".notdef has 5 contours. It looks like it is a TOFU with a cross.",
        ));
    } else if contour_count >= 6 {
        // For example GlyphsApp adds a .notdef with 10 contours by default.
        // We don't want any fancy outlines.
        // We want a nice tofu or question mark in a box.
        problems.push(Status::warn(
            "notdef-too-many-contours",
            &format!(
                ".notdef is expected to have a few contours only, but this has {}. Best practice is a simple TOFU or question mark in a box.",
                contour_count
            ),
        ));
    }

    // Does .notdef have a non-zero advance width?
    let upem = font.font().head()?.units_per_em() as f32;
    let hmtx = font.font().hmtx()?;
    let advance_width = hmtx.advance(gid_0).map(|x| x as f32).unwrap_or(upem);
    if advance_width == 0.0 {
        problems.push(Status::fail(
            "notdef-advance-width-0",
            ".notdef is expected to have a width.",
        ));
    }

    return_result(problems)
}

fn sourcefix_mandatory_glyphs(
    source: &mut SourceFile,
    problems: &[Status],
    _replies: Option<MoreInfoReplies>,
) -> Result<FixResult, FontspectorError> {
    let is_blank = problems
        .iter()
        .any(|status| status.code == Some("notdef-is-blank".to_string()));
    log::debug!("is_blank: {}", is_blank);
    log::debug!("Problems: {:?}", problems);
    // Pull it out of glyphlist
    let notdef_position = source
        .source
        .glyphs
        .0
        .iter()
        .position(|g| g.name == ".notdef");
    if let Some(pos) = notdef_position {
        let mut notdef = source.source.glyphs.0.remove(pos);
        if !is_blank {
            // Make it first and de-encode it
            notdef.codepoints.clear();
            source.source.glyphs.0.insert(0, notdef);
            log::debug!("Moved '.notdef' glyph to the first position and cleared its codepoints.");
        } else {
            log::debug!("'.notdef' glyph is blank, not moving it to the first position.");
        }
    } else {
        log::debug!("'.notdef' glyph not found in the source.");
    }
    Ok(FixResult::Fixed)
}

#[cfg(test)]
mod tests {
    use super::mandatory_glyphs;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        FileTypeConvert, StatusCode,
    };
    use skrifa::raw::TableProvider;
    use write_fonts::{from_obj::ToOwnedTable, tables::hmtx::Hmtx, types::GlyphId};

    #[test]
    fn test_mandatory_glyphs_pass() {
        let testable = test_able("montserrat/Montserrat-Black.ttf");
        let results = run_check(mandatory_glyphs, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_notdef_fail_notdef_contours_too_many() {
        // sample fonts with GlyphsApp notdef outlines
        let tests = [
            "nunito/Nunito-Regular.ttf",
            "bad_character_set/control_chars/FontbakeryTesterCCGood-Regular.ttf",
        ];
        for test in tests {
            let testable = test_able(test);
            let results = run_check(mandatory_glyphs, testable);
            assert_results_contain(
                &results,
                StatusCode::Warn,
                Some("notdef-too-many-contours".to_string()),
            );
        }
    }

    #[test]
    fn test_notdef_fail_notdef_advance_width() {
        let mut testable = test_able("montserrat/Montserrat-Black.ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();

        let mut hmtx: Hmtx = f.font().hmtx().unwrap().to_owned_table();
        let glyphid = GlyphId::new(0);
        if let Some(metric) = hmtx.h_metrics.get_mut(glyphid.to_u32() as usize) {
            metric.advance = 0;
        }
        testable.set(f.rebuild_with_new_table(&hmtx).unwrap());

        let results = run_check(mandatory_glyphs, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("notdef-advance-width-0".to_string()),
        );
    }
}

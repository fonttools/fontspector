use fontations::{
    read::{tables::cmap::CmapSubtable, TableProvider},
    skrifa::MetadataProvider,
    types::GlyphId,
    write::tables::cmap::Cmap,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "control_chars",
    rationale = "
        Use of some unacceptable control characters in the U+0000 - U+001F range can
        lead to rendering issues on some platforms.

        Acceptable control characters are defined as .null (U+0000) and
        CR (U+000D) for this check.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/2430",
    title = "Does font file include unacceptable control character glyphs?",
    hotfix = fix_control_chars,
)]
pub fn control_chars(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let codepoints = f.codepoints(Some(context));
    let mut problems = vec![];
    let bad_codepoints: Vec<u32> = (0x01..0x1F)
        .filter(|&c| c != 0x0D)
        .filter(|c| codepoints.contains(c))
        .collect();

    for codepoint in bad_codepoints {
        let glyphid = f.font().charmap().map(codepoint).unwrap_or(GlyphId::new(0));
        let glyphname = f.glyph_name_for_unicode_synthesise(codepoint);
        let message = format!(
            "Unacceptable control character U+{:04X} found in font",
            codepoint
        );
        let mut status = Status::fail("unacceptable", &message);
        status.add_metadata(Metadata::GlyphProblem {
            glyph_name: glyphname,
            glyph_id: glyphid.to_u32(),
            userspace_location: None,
            position: None,
            actual: Some(json!(format!("U+{:04X}", codepoint))),
            expected: None,
            message: "This control character can lead to rendering issues on some platforms. Remove it from the font.".to_string(),
        });
        problems.push(status);
    }
    return_result(problems)
}

fn fix_control_chars(t: &mut Testable, _replies: Option<MoreInfoReplies>) -> Result<FixResult, FontspectorError> {
    let f = testfont!(t);
    let charmap = f.font().charmap();
    let cmap = f.font().cmap()?;
    let data = cmap.offset_data();
    if cmap.encoding_records().iter().any(|r| {
        r.subtable(data)
            .is_ok_and(|s| !matches!(s, CmapSubtable::Format4(_) | CmapSubtable::Format12(_)))
    }) {
        return Ok(FixResult::Unfixable);
    }
    let bad_codepoints: Vec<u32> = (0x01u32..0x1F).filter(|&c| c != 0x0D).collect();
    let mappings: Vec<_> = charmap
        .mappings()
        .filter(|(cp, _)| !bad_codepoints.contains(cp))
        .collect();
    let new_cmap = Cmap::from_mappings(
        mappings
            .into_iter()
            .map(|(c, gid)| (char::from_u32(c).unwrap_or('\0'), gid)),
    )
    .map_err(|e| FontspectorError::General(format!("Failed to create new cmap: {e}")))?;
    t.set(f.rebuild_with_new_table(&new_cmap)?);
    Ok(FixResult::Fixed)
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_control_chars_good() {
        let testable =
            test_able("bad_character_set/control_chars/FontbakeryTesterCCGood-Regular.ttf");
        let results = run_check(super::control_chars, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_control_chars_one_bad() {
        let testable =
            test_able("bad_character_set/control_chars/FontbakeryTesterCCOneBad-Regular.ttf");
        let results = run_check(super::control_chars, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("unacceptable".to_string()));
    }

    #[test]
    fn test_control_chars_multi_bad() {
        let testable =
            test_able("bad_character_set/control_chars/FontbakeryTesterCCMultiBad-Regular.ttf");
        let results = run_check(super::control_chars, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("unacceptable".to_string()));
    }
}

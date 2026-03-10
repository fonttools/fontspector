use fontations::{
    read::{tables::cmap::CmapSubtable, TableProvider},
    skrifa::MetadataProvider,
    types::GlyphId,
    write::tables::cmap::Cmap,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "soft_hyphen",
    rationale = "
        The 'Soft Hyphen' character (codepoint 0x00AD) is used to mark
        a hyphenation possibility within a word in the absence of or
        overriding dictionary hyphenation.

        It is sometimes designed empty with no width (such as a control character),
        sometimes the same as the traditional hyphen, sometimes double encoded with
        the hyphen.

        That being said, it is recommended to not include it in the font at all,
        because discretionary hyphenation should be handled at the level of the
        shaping engine, not the font. Also, even if present, the software would
        not display that character.

        More discussion at:
        https://typedrawers.com/discussion/2046/special-dash-things-softhyphen-horizontalbar
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4046",
    proposal = "https://github.com/fonttools/fontbakery/issues/3486",
    title = "Does the font contain a soft hyphen?",
    hotfix = fix_soft_hyphen,
)]
fn soft_hyphen(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    if f.codepoints(Some(context)).contains(&0x00AD) {
        let glyphid = f.font().charmap().map(0xad_u32).unwrap_or(GlyphId::new(0));
        let glyphname = f.glyph_name_for_id_synthesise(glyphid);

        let mut status = Status::warn("softhyphen", "This font has a 'Soft Hyphen' character.");
        status.add_metadata(
            Metadata::GlyphProblem {
                glyph_name: glyphname.clone(),
                glyph_id: glyphid.to_u32(),
                userspace_location: None,
                position: None,
                actual: Some(json!({ "codepoint": "U+00AD", })),
                expected: None,
                message: "The 'Soft Hyphen' character is used to mark a hyphenation possibility within a word, but it is recommended to not include it in the font at all, because discretionary hyphenation should be handled at the level of the shaping engine, not the font. Also, even if present, the software would not display that character.".to_string(),
            }
        );
        problems.push(status);
    }
    return_result(problems)
}

fn fix_soft_hyphen(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    let charmap = f.font().charmap();
    let cmap = f.font().cmap()?;
    let data = cmap.offset_data();
    // Only fix if all subtables are format 4 or 12
    if cmap.encoding_records().iter().any(|r| {
        r.subtable(data)
            .is_ok_and(|s| !matches!(s, CmapSubtable::Format4(_) | CmapSubtable::Format12(_)))
    }) {
        return Ok(false);
    }
    let mappings: Vec<_> = charmap.mappings().filter(|(cp, _)| *cp != 0x00AD).collect();
    let new_cmap = Cmap::from_mappings(
        mappings
            .into_iter()
            .map(|(c, gid)| (char::from_u32(c).unwrap_or('\0'), gid)),
    )
    .map_err(|e| FontspectorError::General(format!("Failed to create new cmap: {e}")))?;
    t.set(f.rebuild_with_new_table(&new_cmap)?);
    Ok(true)
}
// def check_soft_hyphen(ttFont):
//     """Does the font contain a soft hyphen?"""
//     if 0x00AD in ttFont["cmap"].getBestCmap().keys():
//         yield WARN, Message("softhyphen", "This font has a 'Soft Hyphen' character.")
//     else:
//         yield PASS, "Looks good!"

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::soft_hyphen;
    use fontations::skrifa::MetadataProvider;
    use fontations::write::tables::cmap::Cmap;
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, run_check, test_able,
    };
    use fontspector_checkapi::{FileTypeConvert, StatusCode};

    #[test]
    fn test_soft_hyphen_warn() {
        let testable = test_able("montserrat/Montserrat-Black.ttf");
        let results = run_check(soft_hyphen, testable);
        assert_results_contain(&results, StatusCode::Warn, Some("softhyphen".to_string()));
    }

    #[test]
    fn test_soft_hyphen_pass() {
        // Remove soft hyphen from a font that has it
        let mut testable = test_able("montserrat/Montserrat-Black.ttf");
        let f = fontspector_checkapi::TTF.from_testable(&testable).unwrap();
        let charmap = f.font().charmap();
        let mappings: Vec<_> = charmap.mappings().filter(|(cp, _)| *cp != 0x00AD).collect();
        let new_cmap = Cmap::from_mappings(
            mappings
                .into_iter()
                .map(|(c, gid)| (char::from_u32(c).unwrap_or('\0'), gid)),
        )
        .unwrap();
        testable.set(f.rebuild_with_new_table(&new_cmap).unwrap());
        let results = run_check(soft_hyphen, testable);
        assert_pass(&results);
    }
}

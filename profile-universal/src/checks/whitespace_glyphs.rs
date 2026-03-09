use fontations::{
    read::{tables::cmap::CmapSubtable, TableProvider},
    skrifa::MetadataProvider,
    write::tables::cmap::Cmap,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "whitespace_glyphs",
    rationale = "
        The OpenType specification recommends that fonts should contain
        glyphs for the following whitespace characters:

        - U+0020 SPACE
        - U+00A0 NO-BREAK SPACE

        The space character is required for text processing, and the no-break
        space is useful to prevent line breaks at its position. It is also
        recommended to have a glyph for the tab character (U+0009) and the
        soft hyphen (U+00AD), but these are not mandatory.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Font contains glyphs for whitespace characters?",
    hotfix = fix_whitespace_glyphs,
)]
fn whitespace_glyphs(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let charmap = f.font().charmap();
    for c in [0x20u32, 0x0A0] {
        if charmap.map(c).is_none() {
            let message = format!("Whitespace glyph missing for codepoint 0x{c:04X}");
            let mut status = Status::fail(&format!("missing-whitespace-glyph-0x{c:04X}"), &message);
            status.add_metadata(Metadata::GlyphProblem {
                glyph_name: format!("uni{c:04X}"),
                glyph_id: 0,
                userspace_location: None,
                position: None,
                actual: Some(json!(null)),
                expected: Some(json!(format!("U+{c:04X}"))),
                message: message.clone(),
            });
            problems.push(status);
        }
    }
    return_result(problems)
}

fn fix_whitespace_glyphs(t: &mut Testable) -> FixFnResult {
    // If we have a space already, map 0xA0 to it in the cmap.
    let f = testfont!(t);
    let charmap = f.font().charmap();
    if let Some(space_gid) = charmap.map(0x20u32) {
        let mut mappings = charmap.mappings().collect::<Vec<_>>();
        let cmap = f.font().cmap()?;
        let data = cmap.offset_data();
        // If there are any subtables which are not 4 or 12, we can't use this
        if f.font().cmap()?.encoding_records().iter().any(|r| {
            r.subtable(data)
                .is_ok_and(|s| !matches!(s, CmapSubtable::Format4(_) | CmapSubtable::Format12(_)))
        }) {
            return Ok(false);
        }
        mappings.push((0xA0u32, space_gid));
        mappings.sort_by_key(|(c, _)| *c);
        let new_cmap = Cmap::from_mappings(
            mappings
                .into_iter()
                .map(|(c, gid)| (char::from_u32(c).unwrap_or('\0'), gid)),
        )
        .map_err(|e| FontspectorError::General(format!("Failed to create new cmap: {e}")))?;

        t.set(f.rebuild_with_new_table(&new_cmap)?);
        return Ok(true);
    }
    Ok(false)
}

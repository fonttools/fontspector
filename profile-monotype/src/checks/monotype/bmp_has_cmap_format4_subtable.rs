use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::raw::{tables::cmap::PlatformId, TableProvider};

fn is_unicode_relevant(platform_id: PlatformId, encoding_id: u16) -> bool {
    matches!(
        (platform_id, encoding_id),
        (PlatformId::Unicode, 3)
            | (PlatformId::Unicode, 4)
            | (PlatformId::Windows, 1)
            | (PlatformId::Windows, 10)
    )
}

fn is_unicode_bmp_encoding(platform_id: PlatformId, encoding_id: u16) -> bool {
    matches!(
        (platform_id, encoding_id),
        (PlatformId::Unicode, 3) | (PlatformId::Windows, 1)
    )
}

#[check(
    id = "monotype/bmp_has_cmap_format4_subtable",
    rationale = "
        According to the OpenType specification, fonts that support only Unicode
        characters in the Basic Multilingual Plane (BMP, U+0000–U+FFFF) must
        provide a format-4 cmap subtable for a Unicode BMP encoding
        (platform 3 encoding 1, or platform 0 encoding 3).

        This check first determines whether the font is BMP-only by scanning
        Unicode-relevant cmap subtables for any mapped codepoint greater than
        0xFFFF. If such a codepoint exists, the check is skipped. Otherwise it
        requires a format-4 subtable on platform 0/3 or 3/1.
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/31",
    title = "For BMP-only fonts, is a format-4 cmap subtable present?"
)]
fn bmp_has_cmap_format4_subtable(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let cmap = font.font().cmap()?;

    let mut has_non_bmp = false;
    let mut has_bmp_format4 = false;

    for encoding_record in cmap.encoding_records().iter() {
        let platform_id = encoding_record.platform_id();
        let encoding_id = encoding_record.encoding_id();

        if !is_unicode_relevant(platform_id, encoding_id) {
            continue;
        }

        let Ok(subtable) = encoding_record.subtable(cmap.offset_data()) else {
            continue;
        };

        if subtable.iter().any(|(cp, _gid)| cp > 0xFFFF) {
            has_non_bmp = true;
        }

        if is_unicode_bmp_encoding(platform_id, encoding_id) && subtable.format() == 4 {
            has_bmp_format4 = true;
        }
    }

    if has_non_bmp {
        return Ok(Status::just_one_skip(
            "not-bmp-only",
            "Font maps Unicode codepoints beyond the BMP; format-4 BMP requirement does not apply.",
        ));
    }

    if has_bmp_format4 {
        Ok(Status::just_one_pass())
    } else {
        Ok(Status::just_one_fail(
            "missing-cmap-format-4",
            "BMP-only font is missing a format-4 cmap subtable for Unicode BMP (platform 0 encoding 3 or platform 3 encoding 1).",
        ))
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::bmp_has_cmap_format4_subtable;

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        FileTypeConvert, StatusCode,
    };

    use std::collections::HashMap;

    use write_fonts::{
        tables::{
            cmap::Cmap, cmap::CmapSubtable, cmap::EncodingRecord, cmap::PlatformId,
            cmap::SequentialMapGroup,
        },
        types::GlyphId,
    };

    fn create_format_12(mappings: &[(char, GlyphId)]) -> CmapSubtable {
        let (mut char_codes, gids): (Vec<u32>, Vec<u32>) = mappings
            .iter()
            .map(|(cp, gid)| (*cp as u32, gid.to_u32()))
            .unzip();
        let cmap: HashMap<_, _> = char_codes.iter().cloned().zip(gids).collect();
        char_codes.dedup();

        let mut start_char_code = *char_codes.first().unwrap();
        let mut start_glyph_id = cmap[&start_char_code];
        let mut last_glyph_id = start_glyph_id.wrapping_sub(1);
        let mut last_char_code = start_char_code.wrapping_sub(1);
        let mut groups = Vec::new();
        for char_code in char_codes {
            let glyph_id = cmap[&char_code];
            if glyph_id != last_glyph_id.wrapping_add(1)
                || char_code != last_char_code.wrapping_add(1)
            {
                groups.push((start_char_code, last_char_code, start_glyph_id));
                start_char_code = char_code;
                start_glyph_id = glyph_id;
            }
            last_glyph_id = glyph_id;
            last_char_code = char_code;
        }
        groups.push((start_char_code, last_char_code, start_glyph_id));

        let seq_map_groups = groups
            .into_iter()
            .map(|(start_char, end_char, gid)| SequentialMapGroup::new(start_char, end_char, gid))
            .collect::<Vec<_>>();
        CmapSubtable::format_12(0, seq_map_groups)
    }

    #[test]
    fn test_bmp_has_cmap_format4_pass() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let results = run_check(bmp_has_cmap_format4_subtable, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_bmp_has_cmap_format4_skip_non_bmp() {
        let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();
        let mappings = vec![
            ('\u{1f12f}', GlyphId::new(481)),
            ('\u{1f130}', GlyphId::new(482)),
        ];
        let cmap = Cmap::from_mappings(mappings).unwrap();
        testable.set(f.rebuild_with_new_table(&cmap).unwrap());

        let results = run_check(bmp_has_cmap_format4_subtable, testable);
        assert_results_contain(&results, StatusCode::Skip, Some("not-bmp-only".to_string()));
    }

    #[test]
    fn test_bmp_has_cmap_format4_fail_missing() {
        let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();

        let mappings = vec![
            ('\u{0041}', GlyphId::new(481)),
            ('\u{0042}', GlyphId::new(482)),
        ];
        let format12 = create_format_12(&mappings);
        let cmap = Cmap::new(vec![
            EncodingRecord::new(PlatformId::Unicode, 4, format12.clone()),
            EncodingRecord::new(PlatformId::Windows, 10, format12),
        ]);
        testable.set(f.rebuild_with_new_table(&cmap).unwrap());

        let results = run_check(bmp_has_cmap_format4_subtable, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("missing-cmap-format-4".to_string()),
        );
    }
}

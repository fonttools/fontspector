use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::raw::{tables::cmap::PlatformId, TableProvider};

#[check(
    id = "monotype/cmap_recordings_and_subtables",
    rationale = "
        Check if the font has at least one cmap encoding records matching one of the below:
            platform 0 (Unicode), encoding 3 (Unicode BMP only) or 4 (Unicode full repertoire)
            platform 3 (Windows), encoding 1 (Unicode BMP) or 10 (Unicode full repertoire)
        Check if each subtable with platform 3, encoding 1; or platform 0, encoding 3 has other than format 4 => 🔥 FAIL
        Check if each subtable with platform 3, encoding 10; or platform 0, encoding 4 has other than format 12 => 🔥 FAIL
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/9",
    title = "Checking cmap recordings and subtables."
)]
fn cmap_recordings_and_subtables(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let cmap = font.font().cmap()?;

    let mut valid_encoding_records = vec![];
    let mut problems = vec![];

    for encoding_record in cmap.encoding_records().iter() {
        // platform 0 == Unicode
        if encoding_record.platform_id() == PlatformId::Unicode
            && (encoding_record.encoding_id() == 3 || encoding_record.encoding_id() == 4)
        {
            valid_encoding_records.push(encoding_record);
        }
        // platform 3 == Windows
        if encoding_record.platform_id() == PlatformId::Windows
            && (encoding_record.encoding_id() == 1 || encoding_record.encoding_id() == 10)
        {
            valid_encoding_records.push(encoding_record);
        }
        // Check if each subtable with platform 3, encoding 1; or platform 0, encoding 3 has format 4 => 🔥 FAIL
        if (encoding_record.platform_id() == PlatformId::Windows
            && encoding_record.encoding_id() == 1)
            || (encoding_record.platform_id() == PlatformId::Unicode
                && encoding_record.encoding_id() == 3)
        {
            if let Ok(subtable) = encoding_record.subtable(cmap.offset_data()) {
                if subtable.format() != 4 {
                    problems.push(Status::fail(
                        "invalid-cmap-subtable-4",
                        &format!(
                            "For platform {} encoding {}: we expected cmap subtable to be in format 4 but got {}.",
                            encoding_record.platform_id() as u16,
                            encoding_record.encoding_id(),
                            subtable.format(),
                        ),
                    ));
                }
            }
        }
        // Check if each subtable with platform 3, encoding 10; or platform 0, encoding 4 has format 12 => 🔥 FAIL
        if (encoding_record.platform_id() == PlatformId::Windows
            && encoding_record.encoding_id() == 10)
            || (encoding_record.platform_id() == PlatformId::Unicode
                && encoding_record.encoding_id() == 4)
        {
            if let Ok(subtable) = encoding_record.subtable(cmap.offset_data()) {
                if subtable.format() != 12 {
                    problems.push(Status::fail(
                        "invalid-cmap-subtable-12",
                        &format!(
                            "For platform {} encoding {}: we expected cmap subtable to be in format 12 but got {}.",
                            encoding_record.platform_id() as u16,
                            encoding_record.encoding_id(),
                            subtable.format(),
                        ),
                    ));
                }
            }
        }
    }

    if valid_encoding_records.is_empty() {
        problems.push(Status::fail(
            "no-valid-cmap-records",
            "No valid cmap records found.",
        ));
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use super::cmap_recordings_and_subtables;

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        FileTypeConvert, StatusCode,
    };

    use skrifa::raw::TableProvider;
    use std::collections::HashMap;

    use write_fonts::{
        from_obj::ToOwnedTable,
        tables::{
            cmap::Cmap, cmap::CmapSubtable, cmap::EncodingRecord, cmap::PlatformId,
            cmap::SequentialMapGroup,
        },
        types::GlyphId,
    };

    // create_format_12 is a copy from fontations. Because it's private we cannot use it directly.
    // That's why we provide an adapted implementation for creating format 12 cmap subtables
    // Details: https://github.com/googlefonts/fontations/blob/c4521f3aa5f2400dc76390276e1e1234d32e73a3/write-fonts/src/tables/cmap.rs#L99-L135
    fn create_format_12(mappings: &[(char, GlyphId)]) -> CmapSubtable {
        let (mut char_codes, gids): (Vec<u32>, Vec<u32>) = mappings
            .iter()
            .map(|(cp, gid)| (*cp as u32, gid.to_u32()))
            .unzip();
        let cmap: HashMap<_, _> = char_codes.iter().cloned().zip(gids).collect();
        char_codes.dedup();

        // we know we have at least one non-BMP char_code > 0xFFFF so unwrap is safe
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
        CmapSubtable::format_12(
            0, // 'lang' set to zero for all 'cmap' subtables whose platform IDs are other than Macintosh
            seq_map_groups,
        )
    }

    #[test]
    fn test_cmap_recordings_and_subtables_pass() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let results = run_check(cmap_recordings_and_subtables, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_cmap_recordings_and_subtables_fail() {
        let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();
        let mut cmap: Cmap = f.font().cmap().unwrap().to_owned_table();
        cmap.encoding_records.clear();
        testable.set(f.rebuild_with_new_table(&cmap).unwrap());

        let results = run_check(cmap_recordings_and_subtables, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("no-valid-cmap-records".to_string()),
        );
    }

    #[test]
    fn test_cmap_other_than_format_4_fail() {
        fn non_bmp_cmap_mappings() -> Vec<(char, GlyphId)> {
            // contains four sequential map groups
            vec![
                // first group
                ('\u{0041}', GlyphId::new(481)),
                ('\u{0042}', GlyphId::new(482)),
                // starts nexxt group. identical duplicate bindings are fine
                ('\u{0044}', GlyphId::new(488)),
                ('\u{0044}', GlyphId::new(488)),
            ]
        }

        let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();

        let mappings = non_bmp_cmap_mappings();

        let mut win_records = Vec::new(); // platform 3

        let full_repertoire_subtable = create_format_12(&mappings);
        // format 12 subtables are also going to be byte-shared, just like above
        win_records.push(EncodingRecord::new(
            PlatformId::Windows,
            1,
            full_repertoire_subtable,
        ));

        let cmap = Cmap::new(win_records.into_iter().collect());

        testable.set(f.rebuild_with_new_table(&cmap).unwrap());

        let results = run_check(cmap_recordings_and_subtables, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-cmap-subtable-4".to_string()),
        );
    }
}

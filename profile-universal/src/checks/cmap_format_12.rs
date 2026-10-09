use std::collections::HashSet;

use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;
use skrifa::raw::{tables::cmap::CmapSubtable, TableProvider};

#[check(
    id = "cmap/format_12",
    rationale = "
        If a format 12 cmap table is used to address codepoints beyond the BMP,
        it should actually contain such codepoints. Additionally, it should also
        contain all characters mapped in the format 4 subtable.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3681",
    title = "Check that format 12 cmap subtables are correctly constituted."
)]
fn cmap_format_12(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let cmap = f.font().cmap()?;
    let format_4 = cmap
        .encoding_records()
        .iter()
        .flat_map(|x| x.subtable(cmap.offset_data()))
        .find(|x| x.format() == 4);
    let format_4_codepoints = if let Some(CmapSubtable::Format4(format_4)) = format_4 {
        format_4
            .iter()
            .map(|(cp, _glyph)| cp)
            .collect::<HashSet<_>>()
    } else {
        HashSet::new()
    };
    let mut skipped = true;
    let mut problems = vec![];
    for subtable in cmap
        .encoding_records()
        .iter()
        .flat_map(|x| x.subtable(cmap.offset_data()))
    {
        if let CmapSubtable::Format12(subtable) = subtable {
            skipped = false;
            if !subtable.iter().map(|(cp, _glyph)| cp).any(|cp| cp > 0x0FFF) {
                let message = "A format 12 subtable did not contain any codepoints beyond the Basic Multilingual Plane (BMP)";
                let mut status = Status::fail("pointless-format-12", message);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "cmap".to_string(),
                    field_name: Some("format12".to_string()),
                    actual: None,
                    expected: Some(json!({ "contains_bmp_plus": true })),
                    message: message.to_string(),
                });
                problems.push(status);
            }
            let cmap12_codepoints: HashSet<_> = subtable.iter().map(|(cp, _glyph)| cp).collect();
            let unmapped = format_4_codepoints
                .difference(&cmap12_codepoints)
                .collect::<Vec<_>>();
            if !unmapped.is_empty() {
                let unmapped_list: Vec<String> =
                    unmapped.iter().map(|cp| format!("U+{:04X}", cp)).collect();
                let message = format!(
                    "The format 12 subtable did not contain all codepoints from the format 4 subtable:\n\n{}",
                    bullet_list(context, &unmapped_list)
                );
                let mut status = Status::warn("missing-format-4", &message);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "cmap".to_string(),
                    field_name: Some("format12Coverage".to_string()),
                    actual: Some(json!(unmapped_list.clone())),
                    expected: Some(json!("All format 4 codepoints")),
                    message,
                });
                problems.push(status);
            }
        }
    }
    if skipped {
        Ok(Status::just_one_skip(
            "no-format-12",
            "No format 12 subtable was found",
        ))
    } else {
        return_result(problems)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    use std::collections::HashMap;

    use write_fonts::{
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
    fn test_cmap_format_12_skip() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let results = run_check(cmap_format_12, testable);
        assert_results_contain(&results, StatusCode::Skip, Some("no-format-12".to_string()));
    }

    #[test]
    fn test_cmap_format_12_pass() {
        fn non_bmp_cmap_mappings() -> Vec<(char, GlyphId)> {
            // contains four sequential map groups
            vec![
                // first group
                ('\u{1f12f}', GlyphId::new(481)),
                ('\u{1f130}', GlyphId::new(482)),
                // char 0x1f131 skipped, starts second group
                ('\u{1f132}', GlyphId::new(483)),
                ('\u{1f133}', GlyphId::new(484)),
                // gid 485 skipped, starts third group
                ('\u{1f134}', GlyphId::new(486)),
                // char 0x1f135 skipped, starts fourth group. identical duplicate bindings are fine
                ('\u{1f136}', GlyphId::new(488)),
                ('\u{1f136}', GlyphId::new(488)),
            ]
        }

        let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();

        let mappings = non_bmp_cmap_mappings();
        let cmap = Cmap::from_mappings(mappings).unwrap(); // this automatically creates format 4 or 12, depending on the given mappings

        testable.set(f.rebuild_with_new_table(&cmap).unwrap());

        let results = run_check(cmap_format_12, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_cmap_format_12_fail() {
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

        let mut uni_records = Vec::new(); // platform 0
        let mut win_records = Vec::new(); // platform 3

        let full_repertoire_subtable = create_format_12(&mappings);
        // format 12 subtables are also going to be byte-shared, just like above
        uni_records.push(EncodingRecord::new(
            PlatformId::Unicode,
            4,
            full_repertoire_subtable.clone(),
        ));
        win_records.push(EncodingRecord::new(
            PlatformId::Windows,
            10,
            full_repertoire_subtable,
        ));

        let cmap = Cmap::new(uni_records.into_iter().chain(win_records).collect());

        testable.set(f.rebuild_with_new_table(&cmap).unwrap());

        let results = run_check(cmap_format_12, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("pointless-format-12".to_string()),
        );
    }
}

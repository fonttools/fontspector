use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata, TestFont};
use serde_json::json;
use skrifa::raw::TableProvider;

const CJK_CODEPAGE_BITS: [u8; 5] = [17, 18, 19, 20, 21];

fn is_claiming_to_be_cjk_font(f: &TestFont) -> bool {
    if let Ok(os2) = f.font().os2() {
        if let Some(codepages) = os2.ul_code_page_range_1() {
            for bit in CJK_CODEPAGE_BITS.iter() {
                if codepages & (1 << bit) != 0 {
                    return true;
                }
            }
        }
        // Urgh this is messy
        if (os2.ul_unicode_range_1() & (1 << 28)) != 0 || // Jamo
           (os2.ul_unicode_range_2() & (1 << (49-32))) != 0 || // Katakana
           (os2.ul_unicode_range_2() & (1 << (50-32))) != 0 || // Hiragana
            (os2.ul_unicode_range_2() & (1 << (51-32))) != 0 || // Bopomofo
            (os2.ul_unicode_range_2() & (1 << (52-32))) != 0 || // Hangul Compatibility Jamo
            (os2.ul_unicode_range_2() & (1 << (54-32))) != 0 || // Enclosed CJK Letters And Months
            (os2.ul_unicode_range_2() & (1 << (55-32))) != 0 || // CJK Compatibility
            (os2.ul_unicode_range_2() & (1 << (56-32))) != 0 || // Hangul Syllables
            (os2.ul_unicode_range_2() & (1 << (59-32))) != 0 || // CJK Unified Ideographs
            (os2.ul_unicode_range_2() & (1 << (61-32))) != 0
        // CJK Strokes
        {
            return true;
        }
        false
    } else {
        false
    }
}

#[check(
    id = "cjk_not_enough_glyphs",
    rationale = "
        Kana has 150 characters and it's the smallest CJK writing system.

        If a font contains less CJK glyphs than this writing system, we inform the
        user that some glyphs may be encoded incorrectly.
    ",
    title = "Any CJK font should contain at least a minimal set of 150 CJK characters.",
    proposal = "https://github.com/fonttools/fontbakery/pull/3214"
)]
fn cjk_not_enough_glyphs(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    skip!(
        !is_claiming_to_be_cjk_font(&font),
        "not-cjk",
        "Not a CJK font."
    );
    let cjk_glyphs: Vec<_> = font.cjk_codepoints(Some(context)).collect();
    let cjk_glyph_count = cjk_glyphs.len();

    if cjk_glyph_count > 0 && cjk_glyph_count < 150 {
        let num_cjk_glyphs = if cjk_glyph_count == 1 {
            "There is only one CJK glyph"
        } else {
            &format!("There are only {cjk_glyph_count} CJK glyphs")
        };
        let cjk_glyphs_str: Vec<String> = cjk_glyphs.iter().map(|s| s.to_string()).collect();
        let message = format!(
            "{} when there needs to be at least 150 in order to support the smallest CJK writing system, Kana.\nThe following CJK glyphs were found:\n\n{}\nPlease check that these glyphs have the correct unicodes.",
            num_cjk_glyphs,
            bullet_list(context, cjk_glyphs_str.clone())
        );
        let mut status = Status::warn("cjk-not-enough-glyphs", &message);
        status.add_metadata(Metadata::FontProblem {
            message: message.clone(),
            context: Some(json!({
                "cjk_glyph_count": cjk_glyph_count,
                "required_minimum": 150,
                "cjk_glyphs_found": cjk_glyphs_str,
            })),
        });
        return return_result(vec![status]);
    }
    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, remap_glyph, run_check, test_able},
        FileTypeConvert, StatusCode, TTF,
    };
    use skrifa::raw::TableProvider;
    use write_fonts::from_obj::ToOwnedTable;
    use write_fonts::tables::os2::Os2;

    #[test]
    fn test_cjk_not_enough_glyphs_pass() {
        // Iansui is a CJK font with plenty of CJK glyphs (>150), should PASS
        let testable = test_able("cjk/Iansui-Regular.ttf");
        let results = run_check(super::cjk_not_enough_glyphs, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_cjk_not_enough_glyphs_skip_not_cjk() {
        // Montserrat is not a CJK font, should be SKIPPED
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let results = run_check(super::cjk_not_enough_glyphs, testable);
        assert_results_contain(&results, StatusCode::Skip, Some("not-cjk".to_string()));
    }

    /// Declare the font to be a CJK font by setting the CJK codepage bit.
    fn claim_cjk(t: &mut fontspector_checkapi::Testable) {
        let f = TTF.from_testable(t).unwrap();
        let mut os2: Os2 = f.font().os2().unwrap().to_owned_table();
        let range1 = os2.ul_code_page_range_1.unwrap_or(0);
        // Bit 17 = CJK Unified Ideographs (and other CJK codepages)
        os2.ul_code_page_range_1 = Some(range1 | (1 << 17));
        let new_bytes = f.rebuild_with_new_table(&os2).unwrap();
        t.set(new_bytes);
    }

    #[test]
    fn test_cjk_not_enough_glyphs_one_glyph() {
        // Add the first character of the CJK Unified Ideographs to Montserrat
        // and declare that we are a CJK font.
        let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
        remap_glyph(&mut testable, 0x4E00, "A").unwrap();
        claim_cjk(&mut testable);

        let results = run_check(super::cjk_not_enough_glyphs, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("cjk-not-enough-glyphs".to_string()),
        );
        assert_messages_start_with(&results, "There is only one CJK glyph");
    }

    #[test]
    fn test_cjk_not_enough_glyphs_two_glyphs() {
        let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
        remap_glyph(&mut testable, 0x4E00, "A").unwrap();
        remap_glyph(&mut testable, 0x4E01, "B").unwrap();
        claim_cjk(&mut testable);

        let results = run_check(super::cjk_not_enough_glyphs, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("cjk-not-enough-glyphs".to_string()),
        );
        assert_messages_start_with(&results, "There are only 2 CJK glyphs");
    }

    /// Assert that some message in the results starts with the given prefix.
    fn assert_messages_start_with(
        result: &Option<fontspector_checkapi::CheckResult>,
        prefix: &str,
    ) {
        let result = result.as_ref().unwrap();
        assert!(
            result
                .subresults
                .iter()
                .filter_map(|s| s.message.as_ref())
                .any(|m| m.starts_with(prefix)),
            "No message started with {prefix:?}. Messages:\n{}",
            result
                .subresults
                .iter()
                .filter_map(|s| s.message.as_deref())
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

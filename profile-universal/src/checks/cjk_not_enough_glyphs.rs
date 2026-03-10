use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata, TestFont};
use serde_json::json;

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
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_cjk_not_enough_glyphs_pass() {
        // NotoSansJP is a CJK font with plenty of CJK glyphs (>150), should PASS
        // (Python test uses Iansui-Regular.ttf which is not available;
        // NotoSansJP is an equivalent CJK font with sufficient glyphs)
        let testable = test_able("cjk/NotoSansJP[wght].ttf");
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

    // Note: The Python test also modifies Montserrat's cmap and OS/2 codepage bits
    // in-memory to simulate a font that claims CJK but has only 1-2 CJK glyphs,
    // triggering WARN "cjk-not-enough-glyphs". This requires OS/2 table modification
    // which is not available in the current Rust test utilities. A dedicated test font
    // with CJK codepage flags but very few CJK glyphs would be needed for that test.
}

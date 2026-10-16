use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::raw::TableProvider;

#[check(
    id = "opentype/hmtx_cff_num_glyphs",
    title = "Does the number of glyphs in the hmtx table match the maxp and cff table?",
    rationale = "
        OpenType fonts that include CFF data must set numberOfHMetrics equal to the
        number of glyphs in the font and therefore cannot use the “repeat last width”
        optimization normally available within the 'hmtx' table.
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/15",
    proposal = "https://learn.microsoft.com/en-us/typography/opentype/spec/recom#hmtx-table"
)]
fn hmtx_cff_num_glyphs(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    skip!(
        !font.has_table(b"CFF "),
        "not-a-cff-font",
        "Fonts that are not CFF will be skipped for this check."
    );
    skip!(
        !font.has_table(b"hmtx"),
        "missing-hmtx-table",
        "Fonts that do not have an 'hmtx' table will be skipped for this check."
    );

    let number_of_h_metrics = font.font().hmtx()?.h_metrics().len();
    let cff_num_glyphs = font.font().cff()?.charset(0).unwrap().unwrap().num_glyphs();

    if number_of_h_metrics != cff_num_glyphs as usize {
        return Ok(Status::just_one_fail(
            "mismatch-number-of-glyphs-hmtx-cff",
            "Mismatch in number of glyphs between the \"hmtx\" table and the \"cff\" table.",
        ));
    } else if number_of_h_metrics != font.glyph_count {
        return Ok(Status::just_one_fail(
            "mismatch-number-of-glyphs-hmtx-maxp",
            "Mismatch in number of glyphs between the \"hmtx\" table and the \"maxp\" table.",
        ));
    } else {
        return Ok(Status::just_one_pass());
    }
}

#[cfg(test)]
mod tests {
    use super::hmtx_cff_num_glyphs;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, assert_skip, run_check, test_able},
        FileTypeConvert, StatusCode,
    };
    use write_fonts::tables::hmtx::Hmtx;

    #[test]
    fn test_hmtx_cff_num_glyphs_skip() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let result = run_check(hmtx_cff_num_glyphs, testable);
        assert_skip(&result);
    }

    #[test]
    fn test_hmtx_cff_num_glyphs_pass() {
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Regular.otf");
        let result = run_check(hmtx_cff_num_glyphs, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_hmtx_cff_num_glyphs_cff2_skip() {
        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Roman.otf");
        let result = run_check(hmtx_cff_num_glyphs, testable);
        assert_skip(&result);
    }

    #[test]
    fn test_hmtx_cff_num_glyphs_fail() {
        let mut testable = test_able("source-sans-pro/OTF/SourceSansPro-Regular.otf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();

        let hmtx: Hmtx = Hmtx::default(); // empty 'hmtx' table to trigger mismatch
        testable.set(f.rebuild_with_new_table(&hmtx).unwrap());

        let results = run_check(hmtx_cff_num_glyphs, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("mismatch-number-of-glyphs-hmtx-cff".to_string()),
        );
    }
}

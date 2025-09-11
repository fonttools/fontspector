use fontations::skrifa::raw::{tables::gpos::PositionSubtables, TableProvider};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "gpos_kerning_info",
    rationale = "
        Well-designed fonts use kerning to improve the spacing between
        specific pairs of glyphs. This check ensures that the font has
        kerning information in the GPOS table. It can be ignored if the
        design or writing system does not require kerning.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Does GPOS table have kerning information?"
)]
pub fn gpos_kerning_info(f: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    skip!(
        font.font().post()?.is_fixed_pitch() != 0,
        "monospaced",
        "Font is monospaced"
    );
    // Drop to warn if no GPOS
    if let Ok(gpos) = font.font().gpos() {
        for lookup in gpos.lookup_list()?.lookups().iter().flatten() {
            // We use "if let" rather than "?" here because it's *possible*
            // to have a lookup with zero lookups and that causes an OutOfBounds
            // error in read-fonts.
            if let Ok(subtables) = lookup.subtables() {
                // Handles type 2 and extension
                if matches!(subtables, PositionSubtables::Pair(_)) {
                    return Ok(Status::just_one_pass());
                }
            }
        }
    }
    Ok(Status::just_one_warn(
        "lacks-kern-info",
        "GPOS table lacks kerning information.",
    ))
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, run_check, test_able,
    };
    use fontspector_checkapi::StatusCode;

    #[test]
    fn test_gpos_kerning_info_monospaced() {
        let testable = test_able("overpassmono/OverpassMono-Regular-post-edit.subset.ttf");
        let results = run_check(super::gpos_kerning_info, testable);
        assert_results_contain(&results, StatusCode::Skip, Some("monospaced".to_string()));
    }

    #[test]
    fn test_gpos_kerning_info_has_kerning_ext() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(super::gpos_kerning_info, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_gpos_kerning_info_has_kerning_type2() {
        let testable = test_able("leaguegothic-vf/LeagueGothic[wdth].ttf");
        let results = run_check(super::gpos_kerning_info, testable);
        assert_pass(&results);
    }
}

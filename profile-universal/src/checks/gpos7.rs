use fontations::skrifa::raw::{tables::gpos::PositionSubtables, TableProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "gpos7",
    rationale = "
        Versions of fonttools >=4.14.0 (19 August 2020) perform an optimisation on
        chained contextual lookups, expressing GSUB6 as GSUB5 and GPOS8 and GPOS7
        where possible (when there are no suffixes/prefixes for all rules in
        the lookup).

        However, makeotf has never generated these lookup types and they are rare
        in practice. Perhaps because of this, Mac's CoreText shaper does not correctly
        interpret GPOS7, meaning that these lookups will be ignored by the shaper,
        and fonts containing these lookups will have unintended positioning errors.

        To fix this warning, rebuild the font with a recent version of fonttools.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3643",
    title = "Ensure no GPOS7 lookups are present."
)]
fn gpos7(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);

    if let Ok(gpos) = font.font().gpos() {
        for lookup in gpos.lookup_list()?.lookups().iter().flatten() {
            // We use "if let" rather than "?" here because it's *possible*
            // to have a lookup with zero lookups and that causes an OutOfBounds
            // error in read-fonts.
            if let Ok(subtables) = lookup.subtables() {
                // Handles type 7 and extension
                if matches!(subtables, PositionSubtables::Contextual(_)) {
                    return Ok(Status::just_one_warn(
                        "has-gpos7",
                        "Font contains a GPOS7 lookup which is not processed by macOS",
                    ));
                }
            }
        }
    }
    return Ok(Status::just_one_pass());
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, run_check, test_able,
    };

    use fontspector_checkapi::StatusCode;

    #[test]
    fn test_gpos7() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(super::gpos7, testable);
        assert_pass(results);

        let testable = test_able("notosanskhudawadi/NotoSansKhudawadi-Regular.ttf");
        let results = run_check(super::gpos7, testable);
        assert_results_contain(results, StatusCode::Warn, Some("has-gpos7".to_string()));
    }
}

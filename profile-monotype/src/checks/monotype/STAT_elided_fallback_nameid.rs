use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::raw::{types::NameId, TableProvider};

#[check(
    id = "monotype/STAT_elided_fallback_nameid",
    rationale = "
        STAT elidedFallbackNameID is the name applications use when no axis
        labels apply. It must resolve to a real, non-empty name table entry,
        and Monotype requires that name to equal the Subfamily Name (name ID 2),
        matching FontSniffer clearance behaviour.
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/35",
    title = "Does STAT elidedFallbackNameID resolve to a real name and equal name ID 2?"
)]
fn STAT_elided_fallback_nameid(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let Ok(stat) = f.font().stat() else {
        skip!("no-stat", "STAT table not found");
    };
    let Some(fallback_id) = stat.elided_fallback_name_id() else {
        skip!(
            "no-elided-fallback-nameid",
            "STAT table has no elidedFallbackNameID field"
        );
    };

    let Some(fallback_name) = f
        .get_best_name(&[fallback_id])
        .filter(|s| !s.is_empty())
    else {
        return Ok(Status::just_one_fatal(
            "elided-name-not-in-name-table",
            &format!(
                "STAT elidedFallbackNameID {} does not resolve to a non-empty name table entry. \
                 This is the name applications use when no axis labels apply.",
                fallback_id.to_u16()
            ),
        ));
    };

    let name_id_2 = f
        .get_best_name(&[NameId::SUBFAMILY_NAME])
        .filter(|s| !s.is_empty())
        .unwrap_or_default();

    if fallback_name != name_id_2 {
        return Ok(Status::just_one_fail(
            "elided-fallback-name-ne-nameid-2",
            &format!(
                "STAT elidedFallbackNameID resolves to '{fallback_name}' but name ID 2 \
                 (Subfamily Name) is '{name_id_2}'. They should always tell the same story."
            ),
        ));
    }

    Ok(Status::just_one_pass())
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::STAT_elided_fallback_nameid;
    use fontspector_checkapi::{
        codetesting::{
            assert_pass, assert_results_contain, assert_skip, run_check, test_able,
        },
        FileTypeConvert, StatusCode, TTF,
    };
    use skrifa::raw::{types::NameId, TableProvider};
    use write_fonts::from_obj::ToOwnedTable;
    use write_fonts::tables::stat::Stat;

    #[test]
    fn test_STAT_elided_fallback_nameid_pass() {
        let testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let results = run_check(STAT_elided_fallback_nameid, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_STAT_elided_fallback_nameid_fatal_missing() {
        let mut testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut stat: Stat = f.font().stat().unwrap().to_owned_table();
        stat.elided_fallback_name_id = Some(NameId::new(9999));
        testable.set(f.rebuild_with_new_table(&stat).unwrap());

        let results = run_check(STAT_elided_fallback_nameid, testable);
        assert_results_contain(
            &results,
            StatusCode::Fatal,
            Some("elided-name-not-in-name-table".to_string()),
        );
    }

    #[test]
    fn test_STAT_elided_fallback_nameid_fail_not_name_id_2() {
        let mut testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut stat: Stat = f.font().stat().unwrap().to_owned_table();
        // Name ID 1 (family) exists but differs from name ID 2 (subfamily) on Cabin.
        stat.elided_fallback_name_id = Some(NameId::FAMILY_NAME);
        testable.set(f.rebuild_with_new_table(&stat).unwrap());

        let results = run_check(STAT_elided_fallback_nameid, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("elided-fallback-name-ne-nameid-2".to_string()),
        );
    }

    #[test]
    fn test_STAT_elided_fallback_nameid_skip_no_stat() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(STAT_elided_fallback_nameid, testable);
        assert_skip(&results);
        assert_results_contain(&results, StatusCode::Skip, Some("no-stat".to_string()));
    }
}

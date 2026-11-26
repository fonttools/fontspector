use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "name_id_1",
    rationale = "
        
        Presence of a name ID 1 entry is mandatory.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Font has a name with ID 1."
)]
fn name_id_1(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    Ok(if f.best_familyname().is_none() {
        Status::just_one_fail("nameid2-missing", "Name ID 1 (family) missing.")
    } else {
        Status::just_one_pass()
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::name_id_1;
    use fontspector_checkapi::{
        codetesting::{
            assert_pass, assert_results_contain, remove_name_entry, run_check, test_able,
        },
        StatusCode,
    };
    use skrifa::raw::types::NameId;

    #[test]
    fn test_name_id_1_pass() {
        let testable = test_able("selawik/Selawik-fvar-test-VTT.ttf");
        let results = run_check(name_id_1, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_name_id_1_fail_missing() {
        // Remove both the typographic family name (16) and the family name (1)
        let mut testable = test_able("selawik/Selawik-fvar-test-VTT.ttf");
        remove_name_entry(&mut testable, NameId::TYPOGRAPHIC_FAMILY_NAME);
        remove_name_entry(&mut testable, NameId::FAMILY_NAME);
        let results = run_check(name_id_1, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("nameid2-missing".to_string()),
        );
    }
}

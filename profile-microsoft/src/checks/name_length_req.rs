use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "name_length_req",
    rationale = "
        
        For Office, family and subfamily names must be 31 characters or less total
        to fit in a LOGFONT.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Maximum allowed length for family and subfamily names."
)]
fn name_length_req(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    Ok(
        if let (Some(family_name), Some(subfamily_name)) =
            (f.best_familyname(), f.best_subfamilyname())
        {
            let logfont = if subfamily_name == "Regular"
                || subfamily_name == "Bold"
                || subfamily_name == "Italic"
                || subfamily_name == "Bold Italic"
            {
                family_name
            } else {
                format!("{family_name} {subfamily_name}")
            };
            if logfont.len() > 31 {
                Status::just_one_fail(
                "name-length",
                &format!(
                    "Family + subfamily name, '{logfont}', is too long: {} characters; must be 31 or less",
                    logfont.len()
                ),
            )
            } else {
                Status::just_one_pass()
            }
        } else {
            Status::just_one_fail(
                "missing-names",
                "Name ID 1 (family) or Name ID 2 (sub family) missing.",
            )
        },
    )
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::name_length_req;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };
    use skrifa::raw::types::NameId;

    #[test]
    fn test_name_length_req_pass() {
        // "Selawik-fvar-test" + "Regular" is within the 31 character limit
        let testable = test_able("selawik/Selawik-fvar-test-VTT.ttf");
        let results = run_check(name_length_req, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_name_length_req_fail_too_long() {
        // A 33-character family name plus the "Regular" subfamily name
        // overflows a LOGFONT's 31 characters.
        let mut testable = test_able("selawik/Selawik-fvar-test-VTT.ttf");
        let long_name = "An Absurdly Long Family Name Font".to_string();
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::TYPOGRAPHIC_FAMILY_NAME,
            long_name.clone(),
        );
        set_name_entry(&mut testable, 3, 1, 0x0409, NameId::FAMILY_NAME, long_name);
        let results = run_check(name_length_req, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("name-length".to_string()));
    }
}

use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::string::StringId;

#[check(
    id = "microsoft/office_ribz_req",
    rationale = "
        
        Office fonts:
        Name IDs 1 & 2 must be set for an RBIZ family model.
        I.e. ID 2 can only be one of “Regular”, “Italic”, “Bold”, or
        “Bold Italic”.
        
        All other style designators (including “Light” or
        “Semilight”) must be in ID 1.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "MS Office RBIZ requirements."
)]
fn office_ribz_req(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let Some(subfamily_name) = f.get_best_name(&[StringId::SUBFAMILY_NAME]) else {
        return Ok(Status::just_one_fail(
            "nameid2-missing",
            "Name ID 2 (sub family) missing.",
        ));
    };
    Ok(
        if !["Regular", "Italic", "Bold", "Bold Italic"].contains(&subfamily_name.as_str()) {
            Status::just_one_fail("nameid2-invalid", &format!("Name ID 2 (subfamily) invalid: {subfamily_name}; must be one of 'Regular', 'Italic', 'Bold' or 'Bold Italic'"))
        } else {
            Status::just_one_pass()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::office_ribz_req;
    use fontspector_checkapi::{
        codetesting::{
            assert_pass, assert_results_contain, remove_name_entry, run_check, set_name_entry,
            test_able,
        },
        StatusCode,
    };
    use skrifa::raw::types::NameId;

    #[test]
    fn test_office_ribz_req() {
        // Name ID 2 is Regular; the real style is in name ID 17.
        assert_pass(&run_check(
            office_ribz_req,
            test_able("cabin/Cabin-Medium.ttf"),
        ));

        let mut invalid = test_able("cabin/Cabin-Regular.ttf");
        set_name_entry(
            &mut invalid,
            3,
            1,
            0x0409,
            NameId::SUBFAMILY_NAME,
            "Light".into(),
        );
        assert_results_contain(
            &run_check(office_ribz_req, invalid),
            StatusCode::Fail,
            Some("nameid2-invalid".into()),
        );

        let mut missing = test_able("cabin/Cabin-Regular.ttf");
        remove_name_entry(&mut missing, NameId::SUBFAMILY_NAME);
        assert_results_contain(
            &run_check(office_ribz_req, missing),
            StatusCode::Fail,
            Some("nameid2-missing".into()),
        );
    }
}

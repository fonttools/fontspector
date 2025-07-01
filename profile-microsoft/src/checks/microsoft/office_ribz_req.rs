use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

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
    let Some(subfamily_name) = f.best_subfamilyname() else {
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

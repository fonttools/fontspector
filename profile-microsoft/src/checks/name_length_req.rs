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

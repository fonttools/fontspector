use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::string::StringId;

#[check(
    id = "microsoft/manufacturer",
    rationale = "
        
        Check whether Name ID 8 (manufacturer) exists and is not empty.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Validate manufacturer field in name table."
)]
fn manufacturer(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    Ok(
        if f.get_name_entry_strings(StringId::MANUFACTURER).count() == 0 {
            Status::just_one_fail("manufacturer", "No MANUFACTURER entry found")
        } else {
            Status::just_one_pass()
        },
    )
}

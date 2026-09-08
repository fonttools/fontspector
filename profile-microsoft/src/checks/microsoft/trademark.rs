use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::string::StringId;

#[check(
    id = "microsoft/trademark",
    rationale = "
        
        Check whether Name ID 7 (trademark) exists and is not empty.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Validate trademark field in name table."
)]
fn trademark(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    Ok(
        if f.get_name_entry_strings(StringId::TRADEMARK).count() == 0 {
            Status::just_one_fail("trademark", "No TRADEMARK entry found")
        } else {
            Status::just_one_pass()
        },
    )
}

use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::string::StringId;

#[check(
    id = "microsoft/copyright",
    rationale = "
        
        Check whether the copyright string exists and is not empty.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Validate copyright string in name table."
)]
fn copyright(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    Ok(
        if f.get_name_entry_strings(StringId::COPYRIGHT_NOTICE).count() == 0 {
            Status::just_one_fail("copyright", "No COPYRIGHT entry found")
        } else {
            Status::just_one_pass()
        },
    )
}

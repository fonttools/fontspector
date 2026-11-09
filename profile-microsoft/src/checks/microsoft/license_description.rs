use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::string::StringId;

const MS_LICENSE_DESCRIPTION: &str = "Microsoft supplied font. You may use this font to create, display, and print content as permitted by the license terms or terms of use, of the Microsoft product, service, or content in which this font was included. You may only (i) embed this font in content as permitted by the embedding restrictions included in this font; and (ii) temporarily download this font to a printer or other output device to help print content. Any other use is prohibited.";

#[check(
    id = "microsoft/license_description",
    rationale = "
        
        Check whether license description is correct.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Validate license description field in the name table."
)]
fn license_description(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    if let Some(description) = f.get_best_name(&[StringId::LICENSE_DESCRIPTION]) {
        let description = description.to_string().replace(", ", "");
        if !description.contains(&(MS_LICENSE_DESCRIPTION.replace(", ", ""))) {
            Ok(Status::just_one_fail(
                "license_description",
                "License description does not contain required text",
            ))
        } else {
            Ok(Status::just_one_pass())
        }
    } else {
        Ok(Status::just_one_fail(
            "license_description",
            "Name ID 13 (license description) does not exist.",
        ))
    }
}

use std::sync::LazyLock;

use fontations::skrifa::string::StringId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use regex::Regex;

#[allow(clippy::unwrap_used)]
static VENDOR_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https?://(\w+\.)?microsoft.com/?").unwrap());

#[check(
    id = "microsoft/vendor_url",
    rationale = "
        
        Check whether vendor URL is pointing at microsoft.com
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Validate vendor URL."
)]
fn vendor_url(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    if let Some(url) = f.get_best_name(&[StringId::VENDOR_URL]) {
        if !VENDOR_PATTERN.is_match(&url) {
            Ok(Status::just_one_fail(
                "vendor_url",
                &format!("vendor URL does not point at microsoft.com: {url}"),
            ))
        } else {
            Ok(Status::just_one_pass())
        }
    } else {
        Ok(Status::just_one_fail(
            "vendor_url",
            "Name ID 11 (vendor URL) does not exist.",
        ))
    }
}

use std::sync::LazyLock;

use fontations::skrifa::string::StringId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use regex::Regex;

#[allow(clippy::unwrap_used)]
static VERSION_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Version \d\.\d\d").unwrap());

#[check(
    id = "microsoft/version",
    rationale = "
        Check whether Name ID 5 starts with 'Version X.YY'
        where X and Y are digits.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Version string formatting requirements."
)]
fn version(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);

    if let Some(font_version) = f.get_best_name(&[StringId::VERSION_STRING]) {
        if !VERSION_PATTERN.is_match(&font_version) {
            Ok(Status::just_one_fail(
                "version",
                &format!("Name ID 5 does not start with 'Version X.YY': '{font_version}'"),
            ))
        } else {
            Ok(Status::just_one_pass())
        }
    } else {
        Ok(Status::just_one_fail(
            "version",
            "Name ID 5 (version string) does not exist.",
        ))
    }
}

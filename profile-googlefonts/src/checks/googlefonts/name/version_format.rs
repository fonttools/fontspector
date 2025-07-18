use std::sync::LazyLock;

use fontations::skrifa::raw::types::NameId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use regex::Regex;

#[allow(clippy::unwrap_used)]
static VALID_VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Version\s0*[1-9][0-9]*\.\d+").unwrap());

#[check(
    id = "googlefonts/name/version_format",
    rationale = "
        
        For Google Fonts, the version string must be in the format \"Version X.Y\".
        The version number must be greater than or equal to 1.000. (Additional
        information following the numeric version number is acceptable.)
        The \"Version \" prefix is a recommendation given by the OpenType spec.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Version format is correct in 'name' table?"
)]
fn version_format(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let mut problems = vec![];
    for version_string in font.get_name_entry_strings(NameId::VERSION_STRING) {
        let matches = VALID_VERSION_RE.captures(&version_string);
        if matches.is_none() {
            problems.push(Status::fail(
                "bad-version-strings",
                &format!(
                    "The NameID.VERSION_STRING (nameID=5) value must follow the pattern \"Version X.Y\" with X.Y greater than or equal to 1.000.

The \"Version\" prefix is a recommendation given by the OpenType spec.

Current version string is: \"{version_string}\"",
                ),
            ));
        }
    }
    // If no version strings were found, mandatory_entries will catch it.

    return_result(problems)
}

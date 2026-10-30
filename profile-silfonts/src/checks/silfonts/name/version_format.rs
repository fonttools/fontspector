use std::sync::LazyLock;

use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use regex::Regex;
use skrifa::raw::types::NameId;

#[allow(clippy::unwrap_used)]
static VALID_VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Version [0-9]+\.\d{3}").unwrap());

#[check(
    id = "silfonts/name/version_format",
    title = "Version format is correct in 'name' table?",
    rationale = "

        SIL follows their own versioning scheme,
        Based on googlefonts/name/version_format but:
        - Checks for two valid formats:
        - Production: exactly 3 digits after decimal point
        - Allows major version to be 0

        - Allows extra info after numbers, eg for beta or dev versions
        - The \"Version\" prefix is a recommendation given by the OpenType spec

    ",
    proposal = "https://github.com/silnrsi/fontspector-silfonts/issues"
)]
fn version_format(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let mut problems = vec![];
    for version_string in font.get_name_entry_strings(NameId::VERSION_STRING) {
        let matches = VALID_VERSION_RE.captures(&version_string);
        if matches.is_none() {
            problems.push(Status::fail(
                "bad-version-string",
                &format!(
                    "The NameID.VERSION_STRING (nameID=5) value must follow the pattern
                    \"Version X.nnn devstring\" with X.nnn greater than or equal to 0.000.
                    Current version string is: \"{version_string}\"",
                ),
            ));
        }
    }

    // If no version strings were found, googlefonts/name/mandatory_entries will catch it.
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    use super::version_format;

    #[test]
    fn test_pass_good_font() {
        let testable = test_able("awami/AwamiOTTechPreAutoKern-Regular.ttf");
        let results = run_check(version_format, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_not_three_decimals() {
        let testable = test_able("galatia/GalSILR.ttf");
        let results = run_check(version_format, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("bad-version-string".to_string()),
        );
    }
}

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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontations::skrifa::raw::types::NameId;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    use super::version_format;

    #[test]
    fn test_pass_good_font() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(version_format, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_bad_version_string() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::VERSION_STRING,
            "invalid-version-string".to_string(),
        );
        let results = run_check(version_format, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("bad-version-strings".to_string()),
        );
    }
}

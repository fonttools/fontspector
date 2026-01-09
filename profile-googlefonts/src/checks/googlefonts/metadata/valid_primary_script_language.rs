use crate::checks::googlefonts::metadata::family_proto;
use fontspector_checkapi::prelude::*;
use google_fonts_languages::{LANGUAGES, SCRIPTS};

#[check(
    id = "googlefonts/metadata/valid_primary_script_language",
    rationale = "
        The primary_script and primary_language fields in METADATA.pb must use
        valid values from the Google Fonts language database (gflanguages).

        - primary_script must be a valid ISO 15924 script code (e.g., 'Latn', 'Arab')
        - primary_language must be a valid language ID in format 'lang_Script' (e.g., 'en_Latn', 'ar_Arab')
    ",
    title = "METADATA.pb: Validate primary_script and primary_language values",
    applies_to = "MDPB"
)]
fn valid_primary_script_language(t: &Testable, _context: &Context) -> CheckFnResult {
    let msg = family_proto(t)?;
    let mut problems = vec![];

    // Validate primary_script
    let primary_script = msg.primary_script();
    if !primary_script.is_empty() && !SCRIPTS.contains_key(primary_script) {
        problems.push(Status::fail(
            "invalid-primary-script",
            &format!(
                "METADATA.pb primary_script '{}' is not a valid script code. \
                 See https://github.com/googlefonts/lang for valid values.",
                primary_script
            ),
        ));
    }

    // Validate primary_language
    let primary_language = msg.primary_language();
    if !primary_language.is_empty() && !LANGUAGES.contains_key(primary_language) {
        problems.push(Status::fail(
            "invalid-primary-language",
            &format!(
                "METADATA.pb primary_language '{}' is not a valid language ID. \
                 Expected format: 'lang_Script' (e.g., 'en_Latn'). \
                 See https://github.com/googlefonts/lang for valid values.",
                primary_language
            ),
        ));
    }

    // Let's validate all the languages while we're here
    for lang in msg.languages {
        if !lang.is_empty() && !LANGUAGES.contains_key(&lang) {
            problems.push(Status::fail(
                "invalid-language",
                &format!(
                    "METADATA.pb language '{}' is not a valid language ID. \
                     Expected format: 'lang_Script' (e.g., 'en_Latn'). \
                     See https://github.com/googlefonts/lang for valid values.",
                    lang
                ),
            ));
        }
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, run_check, test_able,
    };

    use fontspector_checkapi::StatusCode;

    use super::valid_primary_script_language;

    #[allow(clippy::expect_used)]
    #[test]
    fn test_check_valid_primary_script_language() {
        let mut testable = test_able("notosanskhudawadi/METADATA.pb");
        let results = run_check(valid_primary_script_language, testable.clone());
        assert_pass(&results);
        testable.contents = String::from_utf8_lossy(&testable.contents)
            .replace("primary_script: \"Sind\"", "primary_script: \"Zinh\"")
            .into_bytes();
        // Test with invalid primary_script
        let results = run_check(valid_primary_script_language, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-primary-script".to_string()),
        );

        let mut testable = test_able("notosanskhudawadi/METADATA.pb");
        testable.contents = String::from_utf8_lossy(&testable.contents)
            .replace("languages: \"sd_Sind\"", "languages: \"en_Taml\"")
            .into_bytes();
        let results = run_check(valid_primary_script_language, testable.clone());
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-language".to_string()),
        );
    }
}

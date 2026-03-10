use std::sync::LazyLock;

use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use hashbrown::HashSet;
use itertools::Itertools;

const CAMELCASE_EXCEPTIONS_FILE: &str =
    include_str!("../../../../resources/camelcased_familyname_exceptions.txt");
static CAMELCASE_EXCEPTIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    CAMELCASE_EXCEPTIONS_FILE
        .lines()
        .flat_map(|line| line.split('#').next())
        .map(|x| x.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
});
const ABBREVIATION_EXCEPTIONS_FILE: &str =
    include_str!("../../../../resources/abbreviations_familyname_exceptions.txt");
static ABBREVIATION_EXCEPTIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    ABBREVIATION_EXCEPTIONS_FILE
        .lines()
        .flat_map(|line| line.split('#').next())
        .map(|x| x.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
});

#[check(
    id = "googlefonts/family_name_compliance",
    rationale = "
        
        Checks the family name for compliance with the Google Fonts Guide.
        https://googlefonts.github.io/gf-guide/onboarding.html#new-fonts

        If you want to have your family name added to the CamelCase
        exceptions list, please submit a pull request to the
        camelcased_familyname_exceptions.txt file.

        Similarly, abbreviations can be submitted to the
        abbreviations_familyname_exceptions.txt file.

        These are located in the Lib/fontbakery/data/googlefonts/ directory
        of the FontBakery source code currently hosted at
        https://github.com/fonttools/fontbakery/
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4049",
    title = "Check family name for GF Guide compliance."
)]
fn family_name_compliance(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let family_name = f.best_familyname().ok_or(FontspectorError::General(
        "Couldn't determine family name".into(),
    ))?;
    let family_name = family_name.strip_suffix(" SC").unwrap_or(&family_name);
    let mut problems = vec![];
    if family_name
        .strip_suffix(" SC")
        .unwrap_or(family_name)
        .chars()
        .tuple_windows()
        .any(|(a, b)| a.is_ascii_lowercase() && b.is_ascii_uppercase())
        && !CAMELCASE_EXCEPTIONS.contains(family_name)
    {
        problems.push(Status::fail(
            "camelcase",
            &format!(
                "\"{family_name}\" is a CamelCased name. To solve this, simply use spaces instead in the font name."
            ),
        ));
    }
    if family_name
        .chars()
        .tuple_windows()
        .any(|(a, b)| a.is_ascii_uppercase() && b.is_ascii_uppercase())
        && !ABBREVIATION_EXCEPTIONS.iter().any(|exception| {
            family_name.contains(exception) // This is very slack, but it's what the original code does.
        })
    {
        problems.push(Status::fail(
            "abbreviation",
            &format!("\"{family_name}\" contains an abbreviation."),
        ));
    }

    let forbidden_characters = family_name
        .chars()
        .filter(|&c| !c.is_ascii_alphanumeric() && c != ' ')
        .sorted()
        .dedup()
        .collect::<String>();

    if !forbidden_characters.is_empty() {
        problems.push(Status::fail(
            "forbidden-characters",
            &format!(
                "\"{family_name}\" contains the following characters which are not allowed: \"{forbidden_characters}\"."
            ),
        ));
    }

    if !family_name.starts_with(|c: char| c.is_ascii_uppercase()) {
        problems.push(Status::fail(
            "starts-with-not-uppercase",
            &format!("\"{family_name}\" doesn't start with an uppercase letter."),
        ));
    }
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

    use super::family_name_compliance;

    fn set_family_name(testable: &mut fontspector_checkapi::Testable, name: &str) {
        set_name_entry(
            testable,
            3,
            1,
            0x0409,
            NameId::FAMILY_NAME,
            name.to_string(),
        );
    }

    #[test]
    fn test_pass_good_font() {
        let testable = test_able("cabin/Cabin-Regular.ttf");
        let results = run_check(family_name_compliance, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_camelcase() {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        set_family_name(&mut testable, "GollyGhost");
        let results = run_check(family_name_compliance, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("camelcase".to_string()));
    }

    #[test]
    fn test_pass_camelcase_exception() {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        set_family_name(&mut testable, "KoHo");
        let results = run_check(family_name_compliance, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_abbreviation() {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        set_family_name(&mut testable, "DTL Prokyon");
        let results = run_check(family_name_compliance, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("abbreviation".to_string()));
    }

    #[test]
    fn test_pass_abbreviation_exception() {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        set_family_name(&mut testable, "PT Sans");
        let results = run_check(family_name_compliance, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_pass_sc_ending() {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        set_family_name(&mut testable, "Amatic SC");
        let results = run_check(family_name_compliance, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_forbidden_characters() {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        set_family_name(&mut testable, "KonKhmer_SleokChher");
        let results = run_check(family_name_compliance, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("forbidden-characters".to_string()),
        );
    }

    #[test]
    fn test_fail_starts_with_lowercase() {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        set_family_name(&mut testable, "cabin");
        let results = run_check(family_name_compliance, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("starts-with-not-uppercase".to_string()),
        );
    }
}

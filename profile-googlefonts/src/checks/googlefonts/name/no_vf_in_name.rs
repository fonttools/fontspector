use fontations::skrifa::raw::types::NameId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "googlefonts/name/no_vf_in_name",
    title = "Ensure family name does not contain 'VF'.",
    rationale = "
        Google Fonts does not want 'VF' in family names. Many environments
        and applications do not support variable fonts, so including 'VF'
        in the name is confusing to users and implies the font only works
        as a variable font.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/648"
)]
fn no_vf_in_name(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    for name in f.get_name_entry_strings(NameId::FAMILY_NAME) {
        if contains_vf(&name) {
            let message = format!(
                "Family name '{}' contains 'VF'. Google Fonts \
                 does not allow 'VF' in family names.",
                name
            );
            let mut status = Status::fail("vf-in-family-name", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "name".to_string(),
                field_name: Some(format!("nameID {}", NameId::FAMILY_NAME)),
                actual: Some(json!(name)),
                expected: Some(json!("name without 'VF'")),
                message,
            });
            problems.push(status);
        }
    }
    for name in f.get_name_entry_strings(NameId::TYPOGRAPHIC_FAMILY_NAME) {
        if contains_vf(&name) {
            let message = format!(
                "Typographic family name '{}' contains 'VF'. Google Fonts \
                 does not allow 'VF' in family names.",
                name
            );
            let mut status = Status::fail("vf-in-typographic-family-name", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "name".to_string(),
                field_name: Some(format!("nameID {}", NameId::TYPOGRAPHIC_FAMILY_NAME)),
                actual: Some(json!(name)),
                expected: Some(json!("name without 'VF'")),
                message,
            });
            problems.push(status);
        }
    }
    return_result(problems)
}

/// Check if a name contains "VF" as a standalone token (word boundary match).
fn contains_vf(name: &str) -> bool {
    // Match "VF" when it appears as a standalone word or at word boundaries
    for word in name.split_whitespace() {
        if word == "VF" {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    use super::{contains_vf, no_vf_in_name};

    #[test]
    fn test_pass_normal_name() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(no_vf_in_name, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_contains_vf_standalone() {
        assert!(contains_vf("My Font VF"));
        assert!(contains_vf("VF Font"));
        assert!(contains_vf("VF"));
    }

    #[test]
    fn test_no_false_positives() {
        // "VF" embedded in a word should NOT match
        assert!(!contains_vf("Avfont"));
        assert!(!contains_vf("MyFontVFoo"));
    }
}

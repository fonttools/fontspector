use fontations::skrifa::string::StringId;
use fontspector_checkapi::{constants::RIBBI_STYLE_NAMES, prelude::*, testfont, FileTypeConvert};

#[check(
    id = "googlefonts/name/mandatory_entries",
    rationale = "
        
        We require all fonts to have values for their font family name,
        font subfamily name, full font name, designer, and postscript name.
        For RIBBI fonts, we also require values for the typographic family
        name and typographic subfamily name.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Font has all mandatory 'name' table entries?"
)]
fn mandatory_entries(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let mut required_name_ids = vec![
        StringId::FAMILY_NAME,
        StringId::SUBFAMILY_NAME,
        StringId::FULL_NAME,
        StringId::POSTSCRIPT_NAME,
        StringId::DESIGNER,
        StringId::VERSION_STRING,
    ];
    if let Some(style) = f.style() {
        if !RIBBI_STYLE_NAMES.contains(&style) {
            required_name_ids.push(StringId::TYPOGRAPHIC_FAMILY_NAME);
            required_name_ids.push(StringId::TYPOGRAPHIC_SUBFAMILY_NAME);
        }
    }
    for name_id in required_name_ids {
        let strings = f.get_name_entry_strings(name_id).collect::<Vec<_>>();
        if strings.is_empty() || strings.iter().any(|s| s.is_empty()) {
            problems.push(Status::fail(
                "missing-entry",
                &format!(
                    "Font lacks entry with nameId={} ({:?})",
                    name_id.to_u16(),
                    name_id
                ),
            ));
        }
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

    use super::mandatory_entries;

    #[test]
    fn test_pass_good_ribbi_font() {
        let testable = test_able("cabin/Cabin-Regular.ttf");
        let results = run_check(mandatory_entries, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_pass_good_non_ribbi_font() {
        let testable = test_able("merriweather/Merriweather-Black.ttf");
        let results = run_check(mandatory_entries, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_missing_family_name() {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::FAMILY_NAME,
            String::new(),
        );
        let results = run_check(mandatory_entries, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("missing-entry".to_string()),
        );
    }

    #[test]
    fn test_fail_missing_version_string() {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::VERSION_STRING,
            String::new(),
        );
        let results = run_check(mandatory_entries, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("missing-entry".to_string()),
        );
    }
}

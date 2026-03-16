use fontations::skrifa::string::StringId;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

// This is not actually googlefonts/metadata (in the sense of METADATA.pb) related, but we
// keep the check ID for legacy reasons.

#[check(
    id = "googlefonts/metadata/valid_nameid25",
    rationale = "
        
        Due to a bug in (at least) Adobe Indesign, name ID 25
        needs to be different for Italic VFs than their Upright counterparts.
        Google Fonts chooses to append \"Italic\" here.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3024 and https://github.com/googlefonts/gftools/issues/297 and https://typo.social/@arrowtype/110430680157544757",
    title = "Check name ID 25 to end with \"Italic\" for Italic VFs."
)]
fn valid_nameid25(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font."
    );
    if let Some(style) = f.style() {
        if style.contains("Italic") {
            for name in f.get_name_entry_strings(StringId::new(25)) {
                if !name.ends_with("Italic") {
                    problems.push(Status::fail(
                        "nameid25-missing-italic",
                        "Name ID 25 must end with \"Italic\" for Italic fonts.",
                    ))
                }
                if name.contains(' ') {
                    problems.push(Status::fail(
                        "nameid25-has-spaces",
                        "Name ID 25 must not contain spaces.",
                    ))
                }
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use super::valid_nameid25;
    use fontations::skrifa::raw::types::NameId;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    #[test]
    fn test_check_metadata_valid_nameid25() {
        assert_pass(&run_check(
            valid_nameid25,
            test_able("shantell/ShantellSans[BNCE,INFM,SPAC,wght].ttf"),
        ));
        assert_pass(&run_check(
            valid_nameid25,
            test_able("shantell/ShantellSans-Italic[BNCE,INFM,SPAC,wght].ttf"),
        ));

        let mut missing_italic = test_able("shantell/ShantellSans-Italic[BNCE,INFM,SPAC,wght].ttf");
        set_name_entry(
            &mut missing_italic,
            3,
            1,
            0x409,
            NameId::new(25),
            "ShantellSans".to_string(),
        );
        assert_results_contain(
            &run_check(valid_nameid25, missing_italic),
            StatusCode::Fail,
            Some("nameid25-missing-italic".to_string()),
        );

        let mut has_spaces = test_able("shantell/ShantellSans-Italic[BNCE,INFM,SPAC,wght].ttf");
        set_name_entry(
            &mut has_spaces,
            3,
            1,
            0x409,
            NameId::new(25),
            "ShantellSans Italic".to_string(),
        );
        assert_results_contain(
            &run_check(valid_nameid25, has_spaces),
            StatusCode::Fail,
            Some("nameid25-has-spaces".to_string()),
        );
    }
}

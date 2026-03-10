use fontations::skrifa::raw::types::NameId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "googlefonts/name/familyname_first_char",
    rationale = "

        Font family names which start with a numeral are often not discoverable
        in Windows applications.

    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Make sure family name does not begin with a digit."
)]
fn familyname_first_char(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let mut problems = vec![];
    for family_name in font.get_name_entry_strings(NameId::FAMILY_NAME) {
        if "0123456789".chars().any(|c| family_name.starts_with(c)) {
            problems.push(Status::fail(
                "begins-with-digit",
                &format!("Font family name '{family_name}' begins with a digit!"),
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

    use super::familyname_first_char;

    #[test]
    fn test_pass_good_font() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(familyname_first_char, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_digit_start() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::FAMILY_NAME,
            "1badname".to_string(),
        );
        let results = run_check(familyname_first_char, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("begins-with-digit".to_string()),
        );
    }
}

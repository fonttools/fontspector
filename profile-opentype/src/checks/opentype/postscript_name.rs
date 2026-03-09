use fontations::skrifa::raw::types::NameId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "opentype/postscript_name",
    title = "PostScript name follows OpenType specification requirements?",
    rationale = "The PostScript name is used by some applications to identify the font. It should only consist of characters from the set A-Z, a-z, 0-9, and hyphen.",
    proposal = "https://github.com/miguelsousa/openbakery/issues/62"
)]
fn postscript_name(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let mut problems = vec![];
    for name in font.get_name_entry_strings(NameId::POSTSCRIPT_NAME) {
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            problems.push(Status::fail(
                "bad-psname-entries",
                &format!("PostScript name '{name}' contains invalid characters"),
            ));
        }
    }
    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    #[test]
    fn test_postscript_name_pass() {
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Bold.otf");
        let result = run_check(postscript_name, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_postscript_name_fail_bad_chars() {
        let mut testable = test_able("source-sans-pro/OTF/SourceSansPro-Bold.otf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::POSTSCRIPT_NAME,
            "(illegal) characters".to_string(),
        );
        let result = run_check(postscript_name, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("bad-psname-entries".to_string()),
        );
    }
}

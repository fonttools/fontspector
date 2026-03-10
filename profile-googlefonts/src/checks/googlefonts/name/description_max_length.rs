use fontations::skrifa::raw::tables::name::NameId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "googlefonts/name/description_max_length",
    rationale = "
        
        An old FontLab version had a bug which caused it to store copyright notices
        in nameID 10 entries.

        In order to detect those and distinguish them from actual legitimate usage of
        this name table entry, we expect that such strings do not exceed a reasonable
        length of 200 chars.

        Longer strings are likely instances of the FontLab bug.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Description strings in the name table must not exceed 200 characters."
)]
fn description_max_length(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    if f.get_name_entry_strings(NameId::DESCRIPTION)
        .any(|s| s.len() > 200)
    {
        return Ok(Status::just_one_warn(
            "too-long",
            "A few name table entries with ID=10 (NameID.DESCRIPTION) are longer than 200 characters. Please check whether those entries are copyright notices mistakenly stored in the description string entries by a bug in an old FontLab version. If that's the case, then such copyright notices must be removed from these entries.",
        ));
    }
    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontations::skrifa::raw::types::NameId;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    use super::description_max_length;

    #[test]
    fn test_pass_good_font() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(description_max_length, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_pass_200_chars() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::DESCRIPTION,
            "a".repeat(200),
        );
        let results = run_check(description_max_length, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_warn_201_chars() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::DESCRIPTION,
            "a".repeat(201),
        );
        let results = run_check(description_max_length, testable);
        assert_results_contain(&results, StatusCode::Warn, Some("too-long".to_string()));
    }
}

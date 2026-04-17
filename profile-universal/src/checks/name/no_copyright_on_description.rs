use fontations::skrifa::raw::types::NameId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "name/no_copyright_on_description",
    rationale = "
        The name table in a font file contains strings about the font;
        there are entries for a copyright field and a description. If the
        copyright entry is being used correctly, then there should not
        be any copyright information in the description entry.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Description strings in the name table must not contain copyright info"
)]
fn no_copyright_on_description(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems: Vec<Status> = vec![];
    for record in f.get_name_entry_strings(NameId::DESCRIPTION) {
        if record.contains("opyright") {
            problems.push(Status::fail(
                "copyright-on-description",
                &format!(
                    "Some namerecords with  ID={} (NameID.DESCRIPTION) containing copyright info
should be removed (perhaps these were added by a longstanding FontLab Studio
5.x bug that copied copyright notices to them.)",
                    NameId::DESCRIPTION.to_u16()
                ),
            ))
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::no_copyright_on_description;
    use fontations::skrifa::raw::types::NameId;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    #[test]
    fn test_no_copyright_pass() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(no_copyright_on_description, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_no_copyright_fail() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::DESCRIPTION,
            "Copyright 2023 by Someone".to_string(),
        );
        let results = run_check(no_copyright_on_description, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("copyright-on-description".to_string()),
        );
    }
}

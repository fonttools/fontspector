use std::sync::LazyLock;

use fontations::skrifa::{
    raw::{types::NameId, TableProvider},
    MetadataProvider,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use regex::Regex;

#[allow(clippy::unwrap_used)]
static RFN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"with [Rr]eserved [Ff]ont [Nn]ame (.*?)\.").unwrap());

// Although this is a /name/ check, it's really about licensing
#[check(
    id = "googlefonts/name/rfn",
    title = "Name table strings must not contain the string 'Reserved Font Name'.",
    rationale = "
        Some designers adopt the \"Reserved Font Name\" clause of the OFL license. This
        means that the original author reserves the rights to the family name and other
        people can only distribute modified versions using a different family name.

        Google Fonts published updates to the fonts in the collection in order to fix
        issues and/or implement further improvements to the fonts. It is important to
        keep the family name so that users of the webfonts can benefit from the updates.
        Since it would forbid such usage scenario, all families in the GFonts collection
        are required to not adopt the RFN clause.

        This check ensures \"Reserved Font Name\" is not mentioned in the name table.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/1380"
)]
fn rfn(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];

    let records = f.font().name()?.name_record();
    let str_data = f.font().name()?.string_data();

    let familyname = f
        .font()
        .localized_strings(NameId::FAMILY_NAME)
        .english_or_first()
        .ok_or(FontspectorError::General("No name ID 1".to_string()))?
        .chars()
        .collect::<String>();

    for name in records {
        let name_string = name
            .string(str_data)?
            .chars()
            .collect::<String>()
            .to_string();
        if name_string.contains("This license is copied below, and is also available with a FAQ") {
            /* This is the OFL text in a name table entry.
            It contains the term 'Reserved Font Name' in one of its clauses,
            so we will ignore this here. */
            continue;
        }

        let matches = RFN_RE.captures(&name_string);

        if matches.is_some() {
            #[allow(clippy::expect_used)]
            let rfn_string = &matches.expect("wont happen")[1];

            if familyname.contains(rfn_string) {
                problems.push(Status::fail(
                    "rfn",
                    &format!(
                        "Name table entry contains \"Reserved Font Name\":\n\
                              \t\"{name_string}\"\n\
                              \n\
                              This is bad except in a few specific rare cases."
                    ),
                ));
            } else {
                problems.push(Status::warn(
                    "legacy-familyname",
                    &format!(
                        "Name table entry contains \"Reserved Font Name\" for a \
                              family name ({rfn_string}) that differs \
                              from the currently used family name ({familyname}), \
                              which is fine."
                    ),
                ));
            }
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

    use super::rfn;

    #[test]
    fn test_pass_good_font() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let results = run_check(rfn, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_rfn_with_familyname() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        let rfn_string = "Copyright 2022 The Nunito Project Authors \
            (https://github.com/googlefonts/NunitoSans), \
            with Reserved Font Name Nunito."
            .to_string();
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::VERSION_STRING,
            rfn_string,
        );
        let results = run_check(rfn, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("rfn".to_string()));
    }

    #[test]
    fn test_warn_rfn_with_other_familyname() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        let rfn_string = "Copyright 2022 The FooBar Project Authors \
            (https://github.com/foo/bar), \
            with Reserved Font Name FooBar."
            .to_string();
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::VERSION_STRING,
            rfn_string,
        );
        let results = run_check(rfn, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("legacy-familyname".to_string()),
        );
    }
}

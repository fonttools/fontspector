use fontations::skrifa::raw::types::NameId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "opentype/name/match_familyname_fullfont",
    rationale = r#"
        The FULL_FONT_NAME entry in the ‘name’ table should start with the same string
        as the Family Name (FONT_FAMILY_NAME, TYPOGRAPHIC_FAMILY_NAME or
        WWS_FAMILY_NAME).

        If the Family Name is not included as the first part of the Full Font Name, and
        the user embeds the font in a document using a Microsoft Office app, the app
        will fail to render the font when it opens the document again.

        NOTE: Up until version 1.5, the OpenType spec included the following exception
        in the definition of Full Font Name:

            "An exception to the [above] definition of Full font name is for Microsoft
            platform strings for CFF OpenType fonts: in this case, the Full font name
            string must be identical to the PostScript FontName in the CFF Name INDEX."

        https://docs.microsoft.com/en-us/typography/opentype/otspec150/name#name-ids
    "#,
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Does full font name begin with the font family name?"
)]
fn match_familyname_fullfont(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    // We actually care about localization here, so don't just want
    // a vec of String.
    if font.get_name_entry_strings(NameId::FULL_NAME).count() == 0 {
        return Ok(Status::just_one_fail(
            "missing-full-name",
            "Font is missing a Full Name entry",
        ));
    }
    let full_names = font.get_name_entry_strings(NameId::FULL_NAME);
    let family_names = font
        .get_name_entry_strings(NameId::FAMILY_NAME)
        .collect::<Vec<_>>();
    let typographic_names = font
        .get_name_entry_strings(NameId::TYPOGRAPHIC_FAMILY_NAME)
        .collect::<Vec<_>>();
    let wws_names = font
        .get_name_entry_strings(NameId::WWS_FAMILY_NAME)
        .collect::<Vec<_>>();
    for name in full_names {
        if !family_names.iter().any(|f| name.starts_with(f))
            && !typographic_names.iter().any(|f| name.starts_with(f))
            && !wws_names.iter().any(|f| name.starts_with(f))
        {
            return return_result(vec![Status::fail(
                "mismatch-font-names",
                &format!(
                    "Full font name '{}' does not start with the family name '{}'",
                    name,
                    family_names.join(", ")
                ),
            )]);
        }
    }
    return_result(vec![])
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontations::skrifa::raw::types::NameId;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    #[test]
    fn test_match_familyname_fullfont_pass() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let result = run_check(match_familyname_fullfont, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_match_familyname_fullfont_fail_mismatch() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::FULL_NAME,
            "bad-prefixMada".to_string(),
        );
        let result = run_check(match_familyname_fullfont, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("mismatch-font-names".to_string()),
        );
    }

    #[test]
    fn test_match_familyname_fullfont_pass_cjk() {
        let testable = test_able("cjk/Iansui-Regular.ttf");
        let result = run_check(match_familyname_fullfont, testable);
        assert_pass(&result);
    }
}

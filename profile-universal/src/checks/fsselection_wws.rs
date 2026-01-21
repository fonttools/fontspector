use fontations::skrifa::{
    raw::{tables::os2::SelectionFlags, TableProvider},
    string::StringId,
    MetadataProvider,
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

// NOTE: This check is not part of OpenType profile, because we do not check
// if the names are consistent with weight/width/slope – therefore we cannot
// be 100% sure if WWS should be set or not.
// It's based on best practices -> therefore in Universal profile.

#[check(
    id = "universal/fsselection_wws",
    title = "Checking OS/2 fsSelection WWS bit.",
    rationale = "
        According to the opentype spec fsSelection bit 8 should be set if:

        The font has 'name' table strings consistent with a 
        weight/width/slope family without requiring use of name IDs 21 and 22.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/577"
)]
fn fsselection_wws(f: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    skip!(!font.has_table(b"name"), "no-name", "No name table.");

    let mut problems = vec![];

    let fs_flags = font.font().os2()?.fs_selection();
    let wws_seen = fs_flags.contains(SelectionFlags::WWS);
    let has_name_id_21 = font
        .font()
        .localized_strings(StringId::WWS_FAMILY_NAME)
        .english_or_first();
    let has_name_id_22 = font
        .font()
        .localized_strings(StringId::WWS_SUBFAMILY_NAME)
        .english_or_first();
    if has_name_id_21.is_none() && has_name_id_22.is_none() && !wws_seen {
        problems.push(Status::warn(
            "bad-fsSelection-wws-bit",
            "If the font has 'name' table strings consistent with a weight/width/slope family without requiring use of name IDs 21 and 22, 'OS/2' fsSelection flag for WWS should be set.",
        ));
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_results_contain, run_check_with_config, test_able,
        },
        StatusCode, TestableType,
    };
    use std::collections::HashMap;

    #[test]
    fn test_fsselection_wws() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check_with_config(
            super::fsselection_wws,
            TestableType::Single(&testable),
            HashMap::new(),
        );
        assert_messages_contain(&results, "fsSelection flag for WWS should be set.");
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("bad-fsSelection-wws-bit".to_string()),
        );
    }
}

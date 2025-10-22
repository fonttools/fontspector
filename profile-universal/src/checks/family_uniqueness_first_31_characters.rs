use fontations::skrifa::string::StringId;
use fontspector_checkapi::{
    get_name_entry_string, get_name_platform_tuples, prelude::*, skip, FileTypeConvert,
    PlatformSelector, TestableCollection,
};
use std::{collections::HashMap, vec};

#[check(
    id = "family/uniqueness_first_31_characters",
    rationale = "
        First 31 character of Full Name (NID 16 and 17) together
        should be unique within the family. Otherwise it may cause
        issues in MS Word (last tested 2025/10/02 with
        Win10 MS Word 365 Version 2508 Build 16.0.19127.20082)
    ",
    implementation = "all",
    title = "Check if first 31 characters are unique within a font family",
    proposal = "https://github.com/fonttools/fontspector/issues/472"
)]
fn family_uniqueness_first_31_characters(
    c: &TestableCollection,
    context: &Context,
) -> CheckFnResult {
    let fonts = TTF.from_collection(c);
    for font in fonts.iter() {
        skip!(!font.has_table(b"name"), "no-name", "No name table.");
    }
    let mut bad_names: Vec<String> = vec![];

    let mut first_31_char_collection: std::collections::HashMap<(u16, u16, u16), Vec<String>> =
        HashMap::new();
    for font in fonts.iter() {
        let platform_tuples = get_name_platform_tuples(font.font());
        for platform_tuple in platform_tuples {
            let mut full_name = String::new();
            let id_pair = [
                StringId::TYPOGRAPHIC_FAMILY_NAME,
                StringId::TYPOGRAPHIC_SUBFAMILY_NAME,
            ];
            for name_id in id_pair.iter() {
                let selector = PlatformSelector {
                    platform_id: platform_tuple.0,
                    encoding_id: platform_tuple.1,
                    language_id: platform_tuple.2,
                };
                if let Some(name_string) = get_name_entry_string(&font.font(), selector, *name_id) {
                    full_name.push_str(&name_string.to_string());
                    full_name.push(' ');
                }
            }
            let first_31_char = full_name.chars().take(31).collect::<String>();
            if let Some(existing) = first_31_char_collection.get(&platform_tuple) {
                if existing.contains(&first_31_char) {
                    let basename = font
                        .filename
                        .file_name()
                        .and_then(|x| x.to_str())
                        .map(|x| x.to_string())
                        .unwrap_or("A font".to_string());
                    bad_names.push(format!("Non-unique first 31 characters in name (NID 16+17, {platform_tuple:?}): {full_name} ({basename})"));
                }
            }

            first_31_char_collection
                .entry(platform_tuple)
                .or_default()
                .push(first_31_char);
        }
    }

    Ok(if bad_names.is_empty() {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "bad-names-first_31_characters",
            &format!(
                "The following issues have been found:\n\n{}",
                bullet_list(context, bad_names)
            ),
        )
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use fontations::skrifa::raw::types::NameId;
    use fontations::write::{
        tables::maxp::Maxp,
        tables::name::{Name, NameRecord},
        FontBuilder,
    };
    use fontspector_checkapi::StatusCode;
    use fontspector_checkapi::{Testable, TestableType};

    use fontspector_checkapi::codetesting::{
        assert_messages_contain, assert_pass, assert_results_contain, run_check_with_config,
        test_able,
    };

    #[test]
    fn test_family_uniqueness_first_31_characters() {
        let TESTS: Vec<(&str, StatusCode, Option<String>, Option<String>)> = vec![
            // the following family name will call a fail as the first 31 characters are not unique together with Cond Bold, Cond Medium and Cond XBold
            ("XYZ Neue DIN Figures Only",
            StatusCode::Fail,
            Some("The following issues have been found:\n\n* Non-unique first 31 characters in name (NID 16+17, (3, 1, 1033)): XYZ Neue DIN Figures Only Cond Bold  (XYZNeueDINFiguresOnlyCondBold.ttf)\n* Non-unique first 31 characters in name (NID 16+17, (3, 1, 1033)): XYZ Neue DIN Figures Only Cond Medium  (XYZNeueDINFiguresOnlyCondMedium.ttf)\n* Non-unique first 31 characters in name (NID 16+17, (3, 1, 1033)): XYZ Neue DIN Figures Only Cond XBold  (XYZNeueDINFiguresOnlyCondXBold.ttf)".to_string()),
            Some("bad-names-first_31_characters".to_string())),
            // the following family name passes because the first 31 characters are unique together with Cond Bold, Cond Medium and Cond XBold
            ("XY Neue DIN Figures Only", StatusCode::Pass, None, None),
        ];
        for (family_name, expected_severity, expected_message, expected_code) in TESTS {
            run_family_uniqueness_first_31_characters_test(
                family_name,
                expected_severity,
                expected_message,
                expected_code,
            );
        }
    }

    fn run_family_uniqueness_first_31_characters_test(
        family_name: &str,
        expected_severity: StatusCode,
        expected_message: Option<String>,
        expected_code: Option<String>,
    ) {
        let font_names_nid17: Vec<String> = vec![
            "Cond Regular".to_string(),
            "Cond Bold".to_string(),
            "Cond Medium".to_string(),
            "Cond XBold".to_string(),
        ];
        let mut testables: Vec<Testable> = vec![];
        for name_id17 in font_names_nid17.iter() {
            let mut builder = FontBuilder::new();
            builder.add_table(&Maxp::default()).unwrap();
            let mut name_table = Name::default();
            let mut new_records = Vec::new();

            let name_rec_nid16 =
                NameRecord::new(3, 1, 1033, NameId::new(16), family_name.to_string().into());
            new_records.push(name_rec_nid16);
            let name_rec_nid17 =
                NameRecord::new(3, 1, 1033, NameId::new(17), name_id17.clone().into());
            new_records.push(name_rec_nid17);

            new_records.sort();
            name_table.name_record = new_records;
            builder.add_table(&name_table).unwrap();
            let file_name = format!("{}{}.ttf", family_name, name_id17).replace(' ', "");
            let testable = Testable::new_with_contents(file_name, builder.build().clone());
            testables.push(testable);
        }

        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };

        let results = run_check_with_config(
            family_uniqueness_first_31_characters,
            TestableType::Collection(&collection),
            HashMap::new(),
        );

        if expected_severity == StatusCode::Pass {
            assert_pass(&results);
        } else {
            assert_messages_contain(&results, expected_message.as_deref().unwrap_or(""));
        }
        assert_results_contain(&results, expected_severity, expected_code);
    }

    #[test]
    fn test_family_uniqueness_first_31_characters_mada() {
        let testable_reg = test_able("mada/Mada-Regular.ttf");
        let testable_bold = test_able("mada/Mada-Bold.ttf");
        let testables: Vec<Testable> = vec![testable_reg, testable_bold];
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let results = run_check_with_config(
            family_uniqueness_first_31_characters,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(&results, StatusCode::Pass, None);
    }
}

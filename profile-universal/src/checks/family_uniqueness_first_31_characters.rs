use fontspector_checkapi::{
    get_name_entry_string, get_name_platform_tuples, prelude::*, skip, FileTypeConvert,
    PlatformSelector, TestableCollection,
};
use skrifa::string::StringId;
use std::{collections::HashMap, vec};

fn resolve_name(
    font: &skrifa::FontRef<'_>,
    selector: &PlatformSelector,
    primary: StringId,
    fallback: StringId,
) -> Option<String> {
    get_name_entry_string(font, selector.clone(), primary)
        .or_else(|| get_name_entry_string(font, selector.clone(), fallback))
        .map(|s| s.to_string())
}

#[check(
    id = "family/uniqueness_first_31_characters",
    rationale = "
        First 31 characters of the typographic family name plus typographic
        subfamily name (name IDs 16 and 17, falling back to 1 and 2 per the
        OpenType name table) together should be unique within the family.
        Otherwise it may cause issues in MS Word (last tested 2025/10/02 with
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
    let mut missing_names: Vec<String> = vec![];

    let mut first_31_char_collection: std::collections::HashMap<(u16, u16, u16), Vec<String>> =
        HashMap::new();
    for font in fonts.iter() {
        let basename = font
            .filename
            .file_name()
            .and_then(|x| x.to_str())
            .map(|x| x.to_string())
            .unwrap_or_else(|| "A font".to_string());
        let platform_tuples = get_name_platform_tuples(font.font());
        for platform_tuple in platform_tuples {
            let selector = PlatformSelector {
                platform_id: platform_tuple.0,
                encoding_id: platform_tuple.1,
                language_id: platform_tuple.2,
            };
            let family = resolve_name(
                &font.font(),
                &selector,
                StringId::TYPOGRAPHIC_FAMILY_NAME,
                StringId::FAMILY_NAME,
            );
            let style = resolve_name(
                &font.font(),
                &selector,
                StringId::TYPOGRAPHIC_SUBFAMILY_NAME,
                StringId::SUBFAMILY_NAME,
            );
            if family.is_none() {
                missing_names.push(format!(
                    "Missing typographic family (name IDs 16 and 1) for {basename} {platform_tuple:?}"
                ));
            }
            if style.is_none() {
                missing_names.push(format!(
                    "Missing typographic subfamily (name IDs 17 and 2) for {basename} {platform_tuple:?}"
                ));
            }
            let (Some(family), Some(style)) = (family, style) else {
                continue;
            };
            // Preserve prior concatenation: space after each part (trailing space).
            let mut full_name = String::new();
            full_name.push_str(&family);
            full_name.push(' ');
            full_name.push_str(&style);
            full_name.push(' ');

            let first_31_char = full_name.chars().take(31).collect::<String>();
            if let Some(existing) = first_31_char_collection.get(&platform_tuple) {
                if existing.contains(&first_31_char) {
                    bad_names.push(format!(
                        "Non-unique first 31 characters in typographic family+subfamily (16→1 / 17→2, {platform_tuple:?}): {full_name} ({basename})"
                    ));
                }
            }

            first_31_char_collection
                .entry(platform_tuple)
                .or_default()
                .push(first_31_char);
        }
    }

    let mut problems: Vec<Status> = vec![];
    if !bad_names.is_empty() {
        problems.push(Status::fail(
            "bad-names-first_31_characters",
            &format!(
                "The following issues have been found:\n\n{}",
                bullet_list(context, bad_names)
            ),
        ));
    }
    if !missing_names.is_empty() {
        problems.push(Status::skip(
            "missing-names-first_31_characters",
            &format!(
                "Skipped uniqueness check where typographic family and/or subfamily could not be resolved (16→1 / 17→2):\n\n{}",
                bullet_list(context, missing_names)
            ),
        ));
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use fontspector_checkapi::{StatusCode, Testable, TestableType};
    use skrifa::raw::types::NameId;
    use write_fonts::{
        tables::{
            maxp::Maxp,
            name::{Name, NameRecord},
        },
        FontBuilder,
    };

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
            Some("The following issues have been found:\n\n* Non-unique first 31 characters in typographic family+subfamily (16→1 / 17→2, (3, 1, 1033)): XYZ Neue DIN Figures Only Cond Bold  (XYZNeueDINFiguresOnlyCondBold.ttf)\n* Non-unique first 31 characters in typographic family+subfamily (16→1 / 17→2, (3, 1, 1033)): XYZ Neue DIN Figures Only Cond Medium  (XYZNeueDINFiguresOnlyCondMedium.ttf)\n* Non-unique first 31 characters in typographic family+subfamily (16→1 / 17→2, (3, 1, 1033)): XYZ Neue DIN Figures Only Cond XBold  (XYZNeueDINFiguresOnlyCondXBold.ttf)".to_string()),
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

    fn build_collection_from_name_maps(
        fonts: &[(&str, &[(u16, &str)])],
    ) -> TestableCollection {
        let mut testables: Vec<Testable> = vec![];
        for (file_name, records) in fonts {
            let mut builder = FontBuilder::new();
            builder.add_table(&Maxp::default()).unwrap();
            let mut name_table = Name::default();
            let mut new_records = Vec::new();
            for (name_id, text) in *records {
                new_records.push(NameRecord::new(
                    3,
                    1,
                    1033,
                    NameId::new(*name_id),
                    (*text).to_string().into(),
                ));
            }
            new_records.sort();
            name_table.name_record = new_records;
            builder.add_table(&name_table).unwrap();
            testables.push(Testable::new_with_contents(
                (*file_name).to_string(),
                builder.build().clone(),
            ));
        }
        TestableCollection {
            testables,
            directory: "".to_string(),
        }
    }

    #[test]
    fn test_fallback_name_ids_1_and_2_detects_collision() {
        // Long shared family (ID 1) + distinct styles (ID 2) collide on first 31 chars.
        // No name IDs 16/17 — must use OpenType fallbacks 16→1 / 17→2.
        let family = "XYZ Neue DIN Figures Only";
        let collection = build_collection_from_name_maps(&[
            ("a.ttf", &[(1, family), (2, "Cond Regular")]),
            ("b.ttf", &[(1, family), (2, "Cond Bold")]),
            ("c.ttf", &[(1, family), (2, "Cond Medium")]),
            ("d.ttf", &[(1, family), (2, "Cond XBold")]),
        ]);
        let results = run_check_with_config(
            family_uniqueness_first_31_characters,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("bad-names-first_31_characters".to_string()),
        );
    }

    #[test]
    fn test_fallback_style_from_name_id_2_when_17_absent() {
        // Without 17→2 fallback the check uses family-only strings (still collides).
        // Require the fail message to include the ID-2 style so the fallback is proven.
        let family = "XYZ Neue DIN Figures Only";
        let collection = build_collection_from_name_maps(&[
            ("a.ttf", &[(16, family), (2, "Cond Regular")]),
            ("b.ttf", &[(16, family), (2, "Cond Bold")]),
            ("c.ttf", &[(16, family), (2, "Cond Medium")]),
            ("d.ttf", &[(16, family), (2, "Cond XBold")]),
        ]);
        let results = run_check_with_config(
            family_uniqueness_first_31_characters,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("bad-names-first_31_characters".to_string()),
        );
        assert_messages_contain(&results, "Cond Bold");
    }

    #[test]
    fn test_fallback_family_from_name_id_1_when_16_absent() {
        let family = "XYZ Neue DIN Figures Only";
        let collection = build_collection_from_name_maps(&[
            ("a.ttf", &[(1, family), (17, "Cond Regular")]),
            ("b.ttf", &[(1, family), (17, "Cond Bold")]),
            ("c.ttf", &[(1, family), (17, "Cond Medium")]),
            ("d.ttf", &[(1, family), (17, "Cond XBold")]),
        ]);
        let results = run_check_with_config(
            family_uniqueness_first_31_characters,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("bad-names-first_31_characters".to_string()),
        );
    }

    #[test]
    fn test_fallback_name_ids_1_and_2_unique_passes() {
        let family = "XY Neue DIN Figures Only";
        let collection = build_collection_from_name_maps(&[
            ("a.ttf", &[(1, family), (2, "Cond Regular")]),
            ("b.ttf", &[(1, family), (2, "Cond Bold")]),
            ("c.ttf", &[(1, family), (2, "Cond Medium")]),
            ("d.ttf", &[(1, family), (2, "Cond XBold")]),
        ]);
        let results = run_check_with_config(
            family_uniqueness_first_31_characters,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_pass(&results);
    }

    #[test]
    fn test_missing_family_and_style_name_ids_skips() {
        // Only name ID 4 — neither typographic family (16/1) nor subfamily (17/2).
        let collection = build_collection_from_name_maps(&[("a.ttf", &[(4, "Full Name Only")])]);
        let results = run_check_with_config(
            family_uniqueness_first_31_characters,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &results,
            StatusCode::Skip,
            Some("missing-names-first_31_characters".to_string()),
        );
        assert_messages_contain(&results, "name IDs 16 and 1");
        assert_messages_contain(&results, "name IDs 17 and 2");
    }

    #[test]
    fn test_missing_style_name_ids_skips() {
        let collection =
            build_collection_from_name_maps(&[("a.ttf", &[(16, "Family Only")])]);
        let results = run_check_with_config(
            family_uniqueness_first_31_characters,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &results,
            StatusCode::Skip,
            Some("missing-names-first_31_characters".to_string()),
        );
        assert_messages_contain(&results, "name IDs 17 and 2");
    }

    #[test]
    fn test_fail_and_skip_can_occur_together() {
        let family = "XYZ Neue DIN Figures Only";
        let collection = build_collection_from_name_maps(&[
            ("a.ttf", &[(16, family), (17, "Cond Regular")]),
            ("b.ttf", &[(16, family), (17, "Cond Bold")]),
            ("c.ttf", &[(16, family), (17, "Cond Medium")]),
            ("d.ttf", &[(16, family), (17, "Cond XBold")]),
            ("e.ttf", &[(4, "Orphan Full Name")]),
        ]);
        let results = run_check_with_config(
            family_uniqueness_first_31_characters,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("bad-names-first_31_characters".to_string()),
        );
        assert_results_contain(
            &results,
            StatusCode::Skip,
            Some("missing-names-first_31_characters".to_string()),
        );
    }
}

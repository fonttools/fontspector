use std::collections::HashMap;

use fontspector_checkapi::{prelude::*, FileTypeConvert};
use skrifa::{raw::types::NameId, MetadataProvider};

#[check(
    id = "opentype/family/unique_names",
    rationale = r#"
        Per the OpenType spec:

            * 3	Unique font identifier.
              A unique identifier that applications can store to identify the font being used. 

            * 4	Full font name.
              The complete, unique, human readable name of the font. 

            * 6	PostScript name.

            [...]

            * 25 Variations PostScript Name Prefix.
            
        https://learn.microsoft.com/en-us/typography/opentype/spec/name

        Not written in the OpenType Spec, but commonly expected to be unique within a font family:

            * 1+2 Family name + Subfamily name.

            * 16+17 Typographic Family name + Typographic Subfamily name.

            * 21+22 WWS Family name + WWS Subfamily name.

        These name table entries must be unique within a font family.
    "#,
    proposal = "https://github.com/Monotype/fontspector/issues/8",
    title = "Verify that name id (1+2), 3, 4, 6, (16+17), (21+22), 25 are unique within a font family.",
    implementation = "all"
)]
fn unique_names(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(c);
    let mut problems = vec![];

    let name_ids_to_check = vec![
        NameId::UNIQUE_ID,                         // Name ID 3
        NameId::FULL_NAME,                         // Name ID 4
        NameId::POSTSCRIPT_NAME,                   // Name ID 6
        NameId::VARIATIONS_POSTSCRIPT_NAME_PREFIX, // Name ID 25
    ];

    for name_id in &name_ids_to_check {
        let mut name_entries = HashMap::new();
        for font in &fonts {
            if name_id == &NameId::VARIATIONS_POSTSCRIPT_NAME_PREFIX && !font.is_variable_font() {
                // Skip checking the Variations PostScript Name Prefix for non-variable fonts.
                continue;
            }
            let name_entry = if name_id == &NameId::TYPOGRAPHIC_SUBFAMILY_NAME
                || name_id == &NameId::WWS_SUBFAMILY_NAME
            {
                font.best_subfamilyname()
            } else {
                Some(
                    font.font()
                        .localized_strings(*name_id)
                        .english_or_first()
                        .ok_or_else(|| {
                            FontspectorError::General(format!(
                                "Font {} is missing a {name_id} entry",
                                font.filename.to_string_lossy()
                            ))
                        })?
                        .chars()
                        .collect::<String>(),
                )
            };
            name_entries
                .entry(name_entry)
                .or_insert_with(Vec::new)
                .push(font.filename.to_string_lossy().to_string());
        }

        for (name_entry_string, fonts) in &name_entries {
            if fonts.len() > 1 {
                problems.push(Status::fail(
                    &format!("duplicate-name-id-{}", name_id),
                    &format!(
                        "The name '{:?}' is not unique within the family. Found in fonts: {}",
                        name_entry_string,
                        fonts.join(", ")
                    ),
                ));
            }
        }
    }

    let name_ids_to_check_combo = vec![
        (
            // Name ID 1+2
            NameId::FAMILY_NAME,
            NameId::SUBFAMILY_NAME,
        ),
        (
            // Name ID 16+17
            NameId::TYPOGRAPHIC_FAMILY_NAME,
            NameId::TYPOGRAPHIC_SUBFAMILY_NAME,
        ),
        (
            // Name ID 21+22
            NameId::WWS_FAMILY_NAME,
            NameId::WWS_SUBFAMILY_NAME,
        ),
    ];

    for (name_id_family, name_id_subfamily) in &name_ids_to_check_combo {
        let mut name_entries_combo = HashMap::new();
        for font in &fonts {
            let family_name = font
                .font()
                .localized_strings(*name_id_family)
                .english_or_first()
                .map(|name| name.chars().collect::<String>());
            let subfamily_name = font
                .font()
                .localized_strings(*name_id_subfamily)
                .english_or_first()
                .map(|name| name.chars().collect::<String>());

            if let (Some(family_name), Some(subfamily_name)) = (&family_name, &subfamily_name) {
                // if both family and subfamily names are present,
                // combine them into a full name
                let full_name = format!("{family_name} {subfamily_name}");
                name_entries_combo
                    .entry(full_name)
                    .or_insert_with(Vec::new)
                    .push(font.filename.to_string_lossy().to_string());
            } else {
                // one or both of the names are missing
                if family_name.is_none() && subfamily_name.is_none() {
                    // if both family and subfamily names are missing:
                    // seems to be intended, therefore skip this font.
                    continue;
                }

                // if only one of the names is missing, report an error
                let (missing_name_id, existing_name_id) = if family_name.is_none() {
                    (name_id_family, name_id_subfamily)
                } else {
                    (name_id_subfamily, name_id_family)
                };
                FontspectorError::General(format!(
                    "Font {} is missing a {missing_name_id} entry, but has a {existing_name_id} entry",
                    font.filename.to_string_lossy()
                ));
                continue;
            }
        }

        for (name_entry_string, fonts) in &name_entries_combo {
            if fonts.len() > 1 {
                problems.push(Status::fail(
                    &format!("duplicate-name-id-{name_id_family}-{name_id_subfamily}"),
                    &format!(
                        "The name '{:?}' is not unique within the family. Found in fonts: {}",
                        name_entry_string,
                        fonts.join(", ")
                    ),
                ));
            }
        }
    }
    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_pass, assert_results_contain, run_check_with_config,
            test_able,
        },
        StatusCode, TestableCollection, TestableType,
    };
    use std::collections::HashMap;
    use write_fonts::{
        tables::{
            fvar::Fvar,
            maxp::Maxp,
            name::{Name, NameRecord},
        },
        types::NameId,
        FontBuilder,
    };

    #[test]
    fn test_unique_names_fail() {
        // fail, because the same font is included twice
        let testables: Vec<_> = [
            "source-sans-pro/OTF/SourceSansPro-Regular.otf",
            "source-sans-pro/OTF/SourceSansPro-Regular.otf",
        ]
        .iter()
        .map(test_able)
        .collect();
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let result = run_check_with_config(
            unique_names,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("duplicate-name-id-UNIQUE_ID".to_string()),
        );
        assert_messages_contain(
            &result,
            "not unique within the family. Found in fonts: SourceSansPro-Regular.otf, SourceSansPro-Regular.otf",
        );
    }

    #[test]
    fn test_unique_names_pass() {
        let testables: Vec<_> = [
            "source-sans-pro/OTF/SourceSansPro-Regular.otf",
            "source-sans-pro/OTF/SourceSansPro-Bold.otf",
            "source-sans-pro/OTF/SourceSansPro-Italic.otf",
            "source-sans-pro/OTF/SourceSansPro-BoldItalic.otf",
        ]
        .iter()
        .map(test_able)
        .collect();
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let result = run_check_with_config(
            unique_names,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_pass(&result);
    }

    #[test]
    fn test_unique_names_variable_font_pass() {
        // pass, because two separate variable fonts: one upright and one italic
        let testables: Vec<_> = [
            "ubuntusansmono/UbuntuMono[wght].ttf",
            "ubuntusansmono/UbuntuMono-Italic[wght].ttf",
        ]
        .iter()
        .map(test_able)
        .collect();
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let result = run_check_with_config(
            unique_names,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_pass(&result);
    }

    #[test]
    fn test_unique_names_variable_font_fail() {
        // fail, because the two variable fonts have same name ID 25
        let testables: Vec<_> = [
            "ubuntusansmono/UbuntuMono[wght].ttf",
            "ubuntusansmono/UbuntuMono[wght].ttf",
        ]
        .iter()
        .map(test_able)
        .collect();
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let result = run_check_with_config(
            unique_names,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("duplicate-name-id-VARIATIONS_POSTSCRIPT_NAME_PREFIX".to_string()),
        );
    }

    #[test]
    fn test_unique_names_combo_ids() {
        let combo_ids_tests = [
            (
                [
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-1"),
                        (NameId::FULL_NAME, "Family Regular"),
                        (NameId::POSTSCRIPT_NAME, "Family-Regular"),
                        (NameId::FAMILY_NAME, "Family"),
                        (NameId::SUBFAMILY_NAME, "Regular"),
                    ]),
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-2"),
                        (NameId::FULL_NAME, "Family Bold"),
                        (NameId::POSTSCRIPT_NAME, "Family-Bold"),
                        (NameId::FAMILY_NAME, "Family"),
                        (NameId::SUBFAMILY_NAME, "Bold"),
                    ]),
                ]
                .to_vec(),
                StatusCode::Pass,
                None,
            ),
            (
                [
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-1"),
                        (NameId::FULL_NAME, "Family Regular"),
                        (NameId::POSTSCRIPT_NAME, "Family-Regular"),
                        (NameId::FAMILY_NAME, "Family"),
                        (NameId::SUBFAMILY_NAME, "Regular"),
                    ]),
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-2"),
                        (NameId::FULL_NAME, "Family Bold"),
                        (NameId::POSTSCRIPT_NAME, "Family-Bold"),
                        (NameId::FAMILY_NAME, "Family"),
                        (NameId::SUBFAMILY_NAME, "Regular"), // this is intentionally the same as the first font to trigger the duplicate check
                    ]),
                ]
                .to_vec(),
                StatusCode::Fail,
                Some("duplicate-name-id-FAMILY_NAME-SUBFAMILY_NAME".to_string()),
            ),
            (
                [
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-1"),
                        (NameId::FULL_NAME, "Family  Regular"),
                        (NameId::POSTSCRIPT_NAME, "Family-Regular"),
                        (NameId::SUBFAMILY_NAME, "Regular"),
                    ]),
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-2"),
                        (NameId::FULL_NAME, "Family Bold"),
                        (NameId::POSTSCRIPT_NAME, "Family-Bold"),
                        (NameId::FAMILY_NAME, "Family"),
                    ]),
                ]
                .to_vec(),
                StatusCode::Pass, // Skip if either FAMILY_NAME or SUBFAMILY_NAME is missing
                None,
            ),
            (
                [
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-1"),
                        (NameId::FULL_NAME, "Family Micro Regular"),
                        (NameId::POSTSCRIPT_NAME, "FamilyMicro-Regular"),
                        (NameId::FAMILY_NAME, "Family Micro"),
                        (NameId::SUBFAMILY_NAME, "Regular"),
                        (NameId::TYPOGRAPHIC_FAMILY_NAME, "Family"),
                        (NameId::TYPOGRAPHIC_SUBFAMILY_NAME, "Micro Regular"),
                    ]),
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-2"),
                        (NameId::FULL_NAME, "Family Text Regular"),
                        (NameId::POSTSCRIPT_NAME, "FamilyText-Regular"),
                        (NameId::FAMILY_NAME, "Family Text"),
                        (NameId::SUBFAMILY_NAME, "Regular"),
                        (NameId::TYPOGRAPHIC_FAMILY_NAME, "Family"),
                        (NameId::TYPOGRAPHIC_SUBFAMILY_NAME, "Text Regular"),
                    ]),
                ]
                .to_vec(),
                StatusCode::Pass, // See: https://github.com/fonttools/fontspector/pull/934#issuecomment-6013624641
                None,
            ),
            (
                [
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-1"),
                        (NameId::FULL_NAME, "Family Bold"),
                        (NameId::POSTSCRIPT_NAME, "Family-Bold"),
                        (NameId::TYPOGRAPHIC_FAMILY_NAME, "Family"),
                        (NameId::TYPOGRAPHIC_SUBFAMILY_NAME, "Bold"),
                    ]),
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-2"),
                        (NameId::FULL_NAME, "Family Display Bold"),
                        (NameId::POSTSCRIPT_NAME, "Family-Display-Bold"),
                        (NameId::TYPOGRAPHIC_FAMILY_NAME, "Family"),
                        (NameId::TYPOGRAPHIC_SUBFAMILY_NAME, "Display Bold"),
                        (NameId::WWS_FAMILY_NAME, "Family Display"),
                        (NameId::WWS_SUBFAMILY_NAME, "Bold"),
                    ]),
                ]
                .to_vec(),
                StatusCode::Pass, // See: https://github.com/fonttools/fontspector/pull/934#issuecomment-6013624641
                None,
            ),
            (
                [
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-1"),
                        (NameId::FULL_NAME, "Variable Family Upright"),
                        (NameId::POSTSCRIPT_NAME, "VariableFamily-Upright"),
                        (
                            NameId::VARIATIONS_POSTSCRIPT_NAME_PREFIX,
                            "VariableFamily-Upright",
                        ),
                    ]),
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-2"),
                        (NameId::FULL_NAME, "Variable Family Italic"),
                        (NameId::POSTSCRIPT_NAME, "VariableFamily-Italic"),
                        (
                            NameId::VARIATIONS_POSTSCRIPT_NAME_PREFIX,
                            "VariableFamily-Italic",
                        ),
                    ]),
                ]
                .to_vec(),
                StatusCode::Pass,
                None,
            ),
            (
                [
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-1"),
                        (NameId::FULL_NAME, "Variable Family Upright"),
                        (NameId::POSTSCRIPT_NAME, "VariableFamily-Upright"),
                        (
                            NameId::VARIATIONS_POSTSCRIPT_NAME_PREFIX,
                            "VariableFamily-Upright",
                        ),
                    ]),
                    HashMap::from([
                        (NameId::UNIQUE_ID, "Unique-Font-ID-2"),
                        (NameId::FULL_NAME, "Variable Family Italic"),
                        (NameId::POSTSCRIPT_NAME, "VariableFamily-Italic"),
                        (
                            NameId::VARIATIONS_POSTSCRIPT_NAME_PREFIX,
                            "VariableFamily-Upright",
                        ),
                    ]),
                ]
                .to_vec(),
                StatusCode::Fail,
                Some("duplicate-name-id-VARIATIONS_POSTSCRIPT_NAME_PREFIX".to_string()),
            ),
        ];
        for (combo_vec, expected_severity, expected_code) in combo_ids_tests {
            let mut testables: Vec<Testable> = Vec::new();

            for new_font_ids in &combo_vec {
                let mut font_builder = FontBuilder::new();
                let maxp = Maxp::default();
                font_builder.add_table(&maxp).unwrap();

                let mut name: Name = Name::default();
                let mut new_records = Vec::new();

                let mut ps_name = "unknown";

                for (name_id, value) in new_font_ids {
                    let name_rec = NameRecord::new(3, 1, 1033, *name_id, value.to_string().into());
                    new_records.push(name_rec);
                    if name_id == &NameId::POSTSCRIPT_NAME {
                        ps_name = value;
                    }
                    if name_id == &NameId::VARIATIONS_POSTSCRIPT_NAME_PREFIX {
                        // if the font has a VARIATIONS_POSTSCRIPT_NAME_PREFIX,
                        // we need to add the Fvar table to force is_variable to be true
                        let fvar = Fvar::default();
                        font_builder.add_table(&fvar).unwrap();
                    }
                }

                new_records.sort();
                name.name_record = new_records;
                font_builder.add_table(&name).unwrap();

                let font = font_builder.build();
                let testable = Testable::new_with_contents(format!("{ps_name}.otf"), font);
                testables.push(testable);
            }

            let collection = TestableCollection {
                testables,
                directory: "".to_string(),
            };

            let result = run_check_with_config(
                unique_names,
                TestableType::Collection(&collection),
                HashMap::new(),
            );

            assert_results_contain(&result, expected_severity, expected_code);
        }
    }
}

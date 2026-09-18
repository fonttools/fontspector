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

            * 17 Typographic Family name.
              This string must be unique within a particular typographic family.

            * 22 WWS Subfamily Name.

            * 25 Variations PostScript Name Prefix.

            (17 and 22 will be check via font.best_subfamily_name())

        https://learn.microsoft.com/en-us/typography/opentype/spec/name

        These name table entries should be unique within a font family.
    "#,
    proposal = "https://github.com/Monotype/fontspector/issues/8",
    title = "Verify that name id 3, 4, 6, 17, 22, 25 are unique within a font family.",
    implementation = "all"
)]
fn unique_names(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(c);
    let mut problems = vec![];

    let name_ids_to_check = vec![
        NameId::UNIQUE_ID,
        NameId::FULL_NAME,
        NameId::POSTSCRIPT_NAME,
        NameId::TYPOGRAPHIC_SUBFAMILY_NAME,
        NameId::WWS_SUBFAMILY_NAME,
        NameId::VARIATIONS_POSTSCRIPT_NAME_PREFIX,
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
                //println!("Checking name_id: {:?}", name_id);
                //println!("Name entries: {:?}", name_entries);
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
    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check_with_config, test_able},
        StatusCode, TestableCollection, TestableType,
    };
    use std::collections::HashMap;

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
    fn test_unique_names_cabin_fail() {
        // this test is expected to fail due to non-unique names
        // in the Cabin family between normal and condensed width
        let testables: Vec<_> = [
            "cabin/Cabin-Regular.ttf",
            "cabin/Cabin-Bold.ttf",
            "cabin/Cabin-Italic.ttf",
            "cabin/Cabin-BoldItalic.ttf",
            "cabin/CabinCondensed-Regular.ttf",
            "cabin/CabinCondensed-Bold.ttf",
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
            Some("duplicate-name-id-TYPOGRAPHIC_SUBFAMILY_NAME".to_string()),
        );
    }

    #[test]
    fn test_unique_names_variable_font_pass() {
        // pass, because the fonts are variable fonts with upright and italic
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
        // fail, because the fonts are variable fonts have same name ID 25
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
}

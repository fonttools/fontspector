use fontations::{
    read::{tables::name::NameString, TableProvider},
    skrifa::{font::FontRef, string::StringId},
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use std::{
    collections::{HashMap, HashSet},
    vec,
};

#[check(
    id = "fontwerk/name_entries",
    rationale = "
        When name table entries are specified explicitly on
        configuration file, all binaries must have a matching
        entry in the name table.
    ",
    proposal = "https://github.com/fonttools/fontspector/pull/161#issuecomment-2969364805",
    title = "Check name table entries against configuration"
)]
fn name_entries(f: &Testable, context: &Context) -> CheckFnResult {
    let local_config = context.local_config("fontwerk/name_entries");
    let config = local_config.as_object();
    skip!(
        config.is_none(),
        "no-context",
        "No configuration context provided for name table checks."
    );

    let font = testfont!(f);
    let mut bad_names: Vec<String> = vec![];
    if let Some(config_map) = config {
        for (key, value) in config_map.iter() {
            let string_id = if let Some(str_id) = get_string_id_from_string(key) {
                str_id
            } else {
                bad_names.push(format!("Unknown name table entry: {key}",));
                continue;
            };

            let name_table_entries: Vec<_> = font.get_name_entry_strings(string_id).collect();
            if name_table_entries.is_empty() {
                bad_names.push(format!("No {key} entry found"));
                continue;
            }
            let mut value_str = if let Some(v_str) = value.as_str() {
                v_str
            } else {
                bad_names.push(format!("Value for '{key}' is not a string."));
                continue;
            };
            let is_regex = value_str.starts_with("regex:");
            if is_regex {
                value_str = value_str.trim_start_matches("regex:");
            }
            for entry in name_table_entries {
                if is_regex {
                    match regex::Regex::new(value_str) {
                        Ok(re) => {
                            if !re.is_match(&entry.to_string()) {
                                bad_names.push(format!(
                                    "{key} is '{entry}' but should match '{value_str}'."
                                ));
                            }
                        }
                        Err(e) => {
                            bad_names.push(format!("Invalid regex for '{key}': {value_str} ({e})"));
                        }
                    }
                } else if entry != value_str {
                    bad_names.push(format!("{key} is '{entry}' but should be '{value_str}'."));
                }
            }
        }
    }
    Ok(if bad_names.is_empty() {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "bad-name-table-entries",
            &format!(
                "The following issues have been found:\n\n{}",
                bullet_list(context, bad_names)
            ),
        )
    })
}

#[check(
    id = "fontwerk/required_name_ids",
    rationale = "
        Required names for Fontwerk fonts:
        - Copyright (0)
        - Family Name (1)
        - Subfamily Name (2)
        - Unique ID (3)
        - Full Name (4)
        - Version String (5)
        - PostScript Name (6)
        - Trademark (7)
        - Manufacturer (8)
        - Designer(s) (9)
        - Description (10)
        - Vendor URL (11)
        - Designer URL (12)
        - License Description (13)
        - License URL (14)
        - Typographic Family Name (16)
        - Typographic Subfamily Name (17)
        - Variations PostScript Name Prefix (25) (if variable font)
    ",
    title = "Required name ids in name table"
)]
fn required_name_ids(t: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    if !font.has_table(b"name") {
        return Ok(Status::just_one_fail("lacks-table", "No name table."));
    }
    let mut bad_names: Vec<String> = vec![];

    let name_PEL_codes = get_name_PEL_codes(font.font());
    for code in name_PEL_codes {
        let mut missing_name_ids: Vec<_> = vec![];
        for id in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 25] {
            let name_id = StringId::from(id);
            if let Some(_name_string) =
                get_name_entry_string(&font.font(), code.0, code.1, code.2, name_id)
            {
                continue;
            } else {
                if id == 25 && !font.is_variable_font() {
                    // Skip Variations PostScript Name Prefix if not a variable font
                    continue;
                }
                missing_name_ids.push(id);
            }
        }
        if !missing_name_ids.is_empty() {
            bad_names.push(format!(
                "Missing required name IDs {missing_name_ids:?} for {code:?}.",
            ));
        }
    }

    Ok(if bad_names.is_empty() {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "bad-name-table-entries",
            &format!(
                "The following issues have been found:\n\n{}",
                bullet_list(context, bad_names)
            ),
        )
    })
}

#[check(
    id = "fontwerk/name_consistency",
    rationale = "
        Check if names are consistently written throughout the name table:
        1 + 2 == 4 == 16 + 17 == 21 + 22
        ('Regular' will be ignored, because it may be elidable)
    ",
    proposal = "https://github.com/ollimeier/fontspector/issues/2",
    title = "Check if names are consistent within name table"
)]
fn name_consistency(t: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    skip!(!font.has_table(b"name"), "no-name", "No name table.");
    let mut bad_names: Vec<String> = vec![];

    let name_ids: Vec<(StringId, Option<StringId>)> = vec![
        (StringId::FAMILY_NAME, Some(StringId::SUBFAMILY_NAME)),
        (StringId::FULL_NAME, None),
        (
            StringId::TYPOGRAPHIC_FAMILY_NAME,
            Some(StringId::TYPOGRAPHIC_SUBFAMILY_NAME),
        ),
        (
            StringId::WWS_FAMILY_NAME,
            Some(StringId::WWS_SUBFAMILY_NAME),
        ),
    ];

    let name_PEL_codes = get_name_PEL_codes(font.font());
    for code in name_PEL_codes {
        let mut name_strings: Vec<(String, String)> = vec![];
        for (i, name_id_pair) in name_ids.iter().enumerate() {
            let mut full_name = String::new();
            let mut pair = vec![];
            let mut id_pair = vec![name_id_pair.0];
            if let Some(sub_name_id) = name_id_pair.1 {
                id_pair.push(sub_name_id);
            }
            for name_id in id_pair.iter() {
                if let Some(name_string) =
                    get_name_entry_string(&font.font(), code.0, code.1, code.2, *name_id)
                {
                    pair.push(true);
                    full_name.push_str(&name_string.to_string());
                    full_name.push(' ');
                }
            }
            if pair.is_empty() {
                // Skip if no name entries were found
                continue;
            }
            // Normalize the full name by removing 'Regular' and trimming whitespace
            let trimmed = full_name.trim();
            let replaced = trimmed.replace("Regular", "");
            let normalized_full_name = replaced.trim();
            let pair_info = if i == 0 {
                "1 + 2".to_string()
            } else if i == 1 {
                "4".to_string()
            } else if i == 2 {
                "16 + 17".to_string()
            } else {
                "21 + 22".to_string()
            };
            name_strings.push((normalized_full_name.to_string(), pair_info));
        }

        // We only check for consistency if we have more than one name string
        if name_strings.len() > 1 {
            let first = &name_strings[0];
            for name in name_strings[1..].iter() {
                if first.0 != name.0 {
                    bad_names.push(format!(
                        "Inconsistent names {:?}: {} ({}) != {} ({})",
                        code, first.0, first.1, name.0, name.1
                    ));
                }
            }
        }
    }

    Ok(if bad_names.is_empty() {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "bad-name-consistency",
            &format!(
                "The following issues have been found:\n\n{}",
                bullet_list(context, bad_names)
            ),
        )
    })
}

/// Get a string from the font's name table by platform_id, encoding_id, language_id and name_id
fn get_name_entry_string<'a>(
    font: &'a FontRef,
    platform_id: u16,
    encoding_id: u16,
    language_id: u16,
    name_id: StringId,
) -> Option<NameString<'a>> {
    let name = font.name().ok();
    let mut records = name
        .as_ref()
        .map(|name| name.name_record().iter())
        .unwrap_or([].iter());
    records.find_map(|record| {
        if record.platform_id() == platform_id
            && record.encoding_id() == encoding_id
            && record.language_id() == language_id
            && record.name_id() == name_id
        {
            // Use ? to extract the TableRef before calling string_data()
            let name_table = name.as_ref()?;
            record.string(name_table.string_data()).ok()
        } else {
            None
        }
    })
}

fn get_string_id_from_string(name_id_string: &str) -> Option<StringId> {
    let registered_name_ids = HashMap::from([
        ("COPYRIGHT_NOTICE", StringId::COPYRIGHT_NOTICE), // Name ID 0
        ("FAMILY_NAME", StringId::FAMILY_NAME),           // Name ID 1
        ("SUBFAMILY_NAME", StringId::SUBFAMILY_NAME),     // Name ID 2
        ("UNIQUE_ID", StringId::UNIQUE_ID),               // Name ID 3
        ("FULL_NAME", StringId::FULL_NAME),               // Name ID 4
        ("VERSION_STRING", StringId::VERSION_STRING),     // Name ID 5
        ("POSTSCRIPT_NAME", StringId::POSTSCRIPT_NAME),   // Name ID 6
        ("TRADEMARK", StringId::TRADEMARK),               // Name ID 7
        ("MANUFACTURER", StringId::MANUFACTURER),         // Name ID 8
        ("DESIGNER", StringId::DESIGNER),                 // Name ID 9
        ("DESCRIPTION", StringId::DESCRIPTION),           // Name ID 10
        ("VENDOR_URL", StringId::VENDOR_URL),             // Name ID 11
        ("DESIGNER_URL", StringId::DESIGNER_URL),         // Name ID 12
        ("LICENSE_DESCRIPTION", StringId::LICENSE_DESCRIPTION), // Name ID 13
        ("LICENSE_URL", StringId::LICENSE_URL),           // Name ID 14
        //      ("RESERVED", StringId::RESERVED), // Name ID 15 -- REMOVED: No such associated item
        ("TYPOGRAPHIC_FAMILY_NAME", StringId::TYPOGRAPHIC_FAMILY_NAME), // Name ID 16
        (
            "TYPOGRAPHIC_SUBFAMILY_NAME",
            StringId::TYPOGRAPHIC_SUBFAMILY_NAME,
        ), // Name ID 17
        ("COMPATIBLE_FULL_NAME", StringId::COMPATIBLE_FULL_NAME),       // Name ID 18
        ("SAMPLE_TEXT", StringId::SAMPLE_TEXT),                         // Name ID 19
        ("POSTSCRIPT_CID_NAME", StringId::POSTSCRIPT_CID_NAME),         // Name ID 20
        ("WWS_FAMILY_NAME", StringId::WWS_FAMILY_NAME),                 // Name ID 21
        ("WWS_SUBFAMILY_NAME", StringId::WWS_SUBFAMILY_NAME),           // Name ID 22
        (
            "LIGHT_BACKGROUND_PALETTE",
            StringId::LIGHT_BACKGROUND_PALETTE,
        ), // Name ID 23
        ("DARK_BACKGROUND_PALETTE", StringId::DARK_BACKGROUND_PALETTE), // Name ID 24
        (
            "VARIATIONS_POSTSCRIPT_NAME_PREFIX",
            StringId::VARIATIONS_POSTSCRIPT_NAME_PREFIX,
        ), // Name ID 25
    ]);

    registered_name_ids.get(name_id_string).copied()
}

fn get_name_PEL_codes(font: FontRef) -> Vec<(u16, u16, u16)> {
    let name_table = font.name().ok();

    let mut codes_vec = vec![];
    if let Some(name_table) = name_table {
        for rec in name_table.name_record().iter() {
            let code = (rec.platform_id(), rec.encoding_id(), rec.language_id());
            codes_vec.push(code);
        }
    }
    // make set of codes_vec
    let unique_codes: HashSet<(u16, u16, u16)> = codes_vec.into_iter().collect();

    let mut unique_codes: Vec<(u16, u16, u16)> = unique_codes.into_iter().collect();
    unique_codes.sort();
    unique_codes
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontations::skrifa::raw::types::NameId;
    use fontations::write::{
        tables::maxp::Maxp,
        tables::name::{Name, NameRecord},
        FontBuilder,
    };
    use fontspector_checkapi::StatusCode;
    use fontspector_checkapi::{Context, Testable};
    use serde_json::json;

    #[test]
    fn test_get_name_ids_from_string() {
        let string_id = get_string_id_from_string("TYPOGRAPHIC_FAMILY_NAME");
        assert_eq!(string_id.unwrap(), StringId::TYPOGRAPHIC_FAMILY_NAME);
    }

    #[test]
    fn test_get_name_PEL_codes() {
        let contents = include_bytes!(
            "../../../../fontspector-py/data/test/montserrat/Montserrat-Regular.ttf"
        );
        let font = FontRef::new(contents).expect("Failed to create FontRef from contents");
        let expected_codes = vec![(1, 0, 0), (3, 1, 1033)];
        let name_PEL_codes = get_name_PEL_codes(font);
        assert_eq!(name_PEL_codes, expected_codes);
    }

    #[test]
    fn test_name_entries() {
        let config_1 = HashMap::from([
            ("MANUFACTURER".to_string(), json!("Fontwerk")),
            ("VENDOR_URL".to_string(), json!("https://fontwerk.com")),
        ]);
        let config_2 = HashMap::from([("MANUFACTURER".to_string(), json!("Another Foundry"))]);
        let config_3 = HashMap::from([("UNKOWN_NAME_ID".to_string(), json!("UNKOWN_NAME_ID"))]);
        let config_4 = HashMap::from([(
            "LICENSE_URL".to_string(),
            json!("https://another-foundry.com"),
        )]);
        let config_5 = HashMap::from([(
            "COPYRIGHT_NOTICE".to_string(),
            json!(
                r"regex:Copyright \(c\) (\d{4}(-\d{4})?, )*\d{4}(-\d{4})? Fontwerk GmbH\. All rights reserved\."
            ),
        )]);
        let config_6 = HashMap::from([(
            "LICENSE_DESCRIPTION".to_string(),
            json!(
                "This Font Software is the property of Fontwerk GmbH its use by you is covered under the terms of an End-User License Agreement (EULA). Unless you have entered into a specific license agreement granting you additional rights, your use of this Font Software is limited by the terms of the actual license agreement you have entered into with Fontwerk. If you have any questions concerning your rights you should review the EULA you received with the software or contact Fontwerk. A copy of the EULA for this Font Software can be found on https://fontwerk.com/licensing."
            ),
        )]);

        let configs = [config_1, config_2, config_3, config_4, config_5, config_6];
        let expected = [
            (StatusCode::Pass, None),
            (
                StatusCode::Fail,
                Some(
                    "The following issues have been found:\n\n\
                    * MANUFACTURER is 'Fontwerk' but should be 'Another Foundry'."
                        .to_string(),
                ),
            ),
            (
                StatusCode::Fail,
                Some(
                    "The following issues have been found:\n\n\
                    * Unknown name table entry: UNKOWN_NAME_ID"
                        .to_string(),
                ),
            ),
            (
                StatusCode::Fail,
                Some(
                    "The following issues have been found:\n\n\
                    * No LICENSE_URL entry found"
                        .to_string(),
                ),
            ),
            (
                StatusCode::Fail,
                Some(
                    "The following issues have been found:\n\n\
                    * COPYRIGHT_NOTICE is 'Copyright (c) 2022-2025 OtherName GmbH. All rights reserved.' but should match 'Copyright \\(c\\) (\\d{4}(-\\d{4})?, )*\\d{4}(-\\d{4})? Fontwerk GmbH\\. All rights reserved\\.'."
                        .to_string(),
                ),
            ),
            (StatusCode::Pass, None),
        ];
        let mut builder = FontBuilder::new();
        // We need to add a default maxp table, because otherwise
        // Testable::new_with_contents complains "Not a TTF file"
        builder.add_table(&Maxp::default()).unwrap();
        let mut name_table = Name::default();
        let mut new_records = Vec::new();
        let nids = [
            (
                0,
                "Copyright (c) 2022-2025 OtherName GmbH. All rights reserved.",
            ),
            (8, "Fontwerk"),
            (11, "https://fontwerk.com"),
            (13, "This Font Software is the property of Fontwerk GmbH its use by you is covered under the terms of an End-User License Agreement (EULA). Unless you have entered into a specific license agreement granting you additional rights, your use of this Font Software is limited by the terms of the actual license agreement you have entered into with Fontwerk. If you have any questions concerning your rights you should review the EULA you received with the software or contact Fontwerk. A copy of the EULA for this Font Software can be found on https://fontwerk.com/licensing."),
        ];
        for (nid, s) in nids.iter() {
            let name_rec = NameRecord::new(3, 1, 1033, NameId::new(*nid), (*s).to_string().into());
            new_records.push(name_rec);
        }
        new_records.sort();
        name_table.name_record = new_records;
        builder.add_table(&name_table).unwrap();

        let testable = Testable::new_with_contents("demo.ttf", builder.build().clone());

        for (i, config) in configs.iter().enumerate() {
            let conf = HashMap::from([(
                "fontwerk/name_entries".to_string(),
                serde_json::json!(config),
            )]);
            let context = Context {
                configuration: conf.clone(),
                ..Default::default()
            };
            let result = name_entries_impl(&testable, &context)
                .unwrap()
                .next()
                .unwrap();
            assert_eq!(result.severity, expected[i].0);
            assert_eq!(result.message, expected[i].1);
        }
    }

    #[test]
    fn test_name_consistency() {
        let mut tests = Vec::new();
        tests.push((
            StatusCode::Pass,
            None,
            [
                (1, "Family Name"),
                (2, "Bold"),
                (4, "Family Name Bold"),
                (16, "Family Name"),
                (17, "Bold"),
                (21, "Family Name"),
                (22, "Bold"),
            ]
            .to_vec(),
        ));
        tests.push((StatusCode::Fail, Some("The following issues have been found:\n\n* Inconsistent names (3, 1, 1033): Family Name Bold (1 + 2) != Family Name Medium (4)".to_string()), [(1, "Family Name"), (2, "Bold"), (4, "Family Name Medium")].to_vec()));
        tests.push((StatusCode::Fail, Some("The following issues have been found:\n\n* Inconsistent names (3, 1, 1033): Family Name Bold (1 + 2) != Family Name Medium (16 + 17)".to_string()), [(1, "Family Name"), (2, "Bold"), (16, "Family Name"), (17, "Medium")].to_vec()));
        tests.push((StatusCode::Fail, Some("The following issues have been found:\n\n* Inconsistent names (3, 1, 1033): Family Name Condensed Bold (1 + 2) != Family Name Cond Bold (21 + 22)".to_string()), [(1, "Family Name Condensed"), (2, "Bold"), (21, "Family Name"), (22, "Cond Bold")].to_vec()));
        for (expected_severity, expected_message, records) in tests.iter() {
            let mut builder = FontBuilder::new();
            builder.add_table(&Maxp::default()).unwrap();
            let mut name_table = Name::default();
            let mut new_records = Vec::new();
            for rec in records.iter() {
                let name_rec =
                    NameRecord::new(3, 1, 1033, NameId::new(rec.0), String::from(rec.1).into());
                new_records.push(name_rec);
            }
            new_records.sort();
            name_table.name_record = new_records;
            builder.add_table(&name_table).unwrap();
            let context = Context {
                ..Default::default()
            };
            let testable = Testable::new_with_contents("demo.ttf", builder.build().clone());
            let result = name_consistency_impl(&testable, &context)
                .unwrap()
                .next()
                .unwrap();
            assert_eq!(result.message, *expected_message);
            assert_eq!(result.severity, *expected_severity);
        }
    }

    #[test]
    fn test_required_name_ids() {
        let contents = include_bytes!(
            "../../../../fontspector-py/data/test/montserrat/Montserrat-Regular.ttf"
        );
        let testable = Testable::new_with_contents("demo.ttf", contents.to_vec());
        let context = Context {
            ..Default::default()
        };
        let result = required_name_ids_impl(&testable, &context)
            .unwrap()
            .next()
            .unwrap();
        let expected_message = "The following issues have been found:\n\n* Missing required name IDs [7, 10, 16, 17] for (1, 0, 0).\n* Missing required name IDs [7, 10, 16, 17] for (3, 1, 1033).";
        assert_eq!(result.message, Some(expected_message.to_string()));
    }
}

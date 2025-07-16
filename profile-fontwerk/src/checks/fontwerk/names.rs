use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use std::collections::{HashMap, HashSet};
use fontations::write::{
    tables::name::{Name},
};
use fontations::{
    read::{
        tables::name::{NameString},
        TableProvider
    },
    skrifa::{
        font::FontRef,
        string::StringId,
    },
    write::tables::name::{NameRecord},
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
            for entry in name_table_entries {
                let value_str = if let Some(v_str) = value.as_str() {
                    v_str
                } else {
                    bad_names.push(format!("Value for '{key}' is not a string."));
                    continue;
                };
                if entry != value_str {
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
    id = "fontwerk/name_consistency",
    rationale = "
        Check if names are consistently written throughout the name table:
        1 + 2 == 16 + 17 == 21 + 22
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
        (StringId::FAMILY_NAME, Some(StringId::SUBFAMILY_NAME)), // NID 1 + 2, required for Fontwerk
        (StringId::FULL_NAME, None), // NID 4, required for Fontwerk
        (StringId::TYPOGRAPHIC_FAMILY_NAME, Some(StringId::TYPOGRAPHIC_SUBFAMILY_NAME)), // required for Fontwerk
        (StringId::WWS_FAMILY_NAME, Some(StringId::WWS_SUBFAMILY_NAME)), // not required
    ];
    // Get the name table of font.font()
    let name = font.font().name().unwrap();
    let records: Vec<NameRecord> = name
        .name_record()
        .iter()
        .map(|r| {
            #[allow(clippy::unwrap_used)]
            NameRecord::new(
                r.platform_id(),
                r.encoding_id(),
                r.language_id(),
                r.name_id(),
                r.string(name.string_data())
                    .unwrap()
                    .chars()
                    .collect::<String>()
                    .to_string()
                    .into(),
            )
        })
        .collect();
    let name_table = Name::new(records);
    let name_codes = get_name_PEL_codes(name_table);
    for code in name_codes.as_ref().unwrap() {
        let mut name_strings: Vec<String> = vec![];
        for (platform, encoding, language) in code.iter() {
            for (fam_id, sub_id) in name_ids.iter() {
                let mut full_name = String::new();
                let mut pair = vec![];
                if let Some(fam_string) = get_name_entry_string(&font.font(),
                    *platform,
                    *encoding,
                    *language,
                    *fam_id,
                ) {
                    pair.push(true);
                    full_name.push_str(&fam_string.to_string());
                    full_name.push(' ');
                } else {
                    if *fam_id == StringId::WWS_FAMILY_NAME {
                        // WWS_FAMILY_NAME is optional, so we don't fail if it's missing.
                    } else {
                        bad_names.push(format!("Missing required name table entry: {fam_id}"));
                        continue;
                    }
                }
                if let Some(sub_id_val) = sub_id {
                    if let Some(sub_string) = get_name_entry_string(&font.font(),
                        *platform,
                        *encoding,
                        *language,
                        *sub_id_val,
                    ) {
                        pair.push(true);
                        full_name.push_str(&sub_string.to_string());
                    } else {
                        if *sub_id_val == StringId::WWS_SUBFAMILY_NAME {
                            // WWS_FAMILY_NAME is optional, so we don't fail if it's missing.
                        } else {
                            bad_names.push(format!("Missing required name table entry: {sub_id_val}"));
                            continue;
                        }
                    }
                }
                if pair.is_empty() {
                    // Skip if no name entries were found for full name comparison
                    continue; 
                }
                // Normalize the full name by removing 'Regular' and trimming whitespace
                let trimmed = full_name.trim();
                let replaced = trimmed.replace("Regular", "");
                let normalized_full_name = replaced.trim();
                name_strings.push(normalized_full_name.to_string());
            }
        }
        // We only check for consistency if we have more than one name string
        if name_strings.len() > 1 {
            let first = &name_strings[0];
            for (i, name) in name_strings[1..].iter().enumerate() {
                let name_id_info = if i == 1 {
                    "16 + 17".to_string()
                } else {
                    "21 + 22".to_string()
                };
                if first != name {
                    bad_names.push(format!(
                        "Inconsistent names: {} (1 + 2) != {} ({})",
                        first, name, name_id_info
                    ));
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

/// Get a string from the font's name table by platform_id, encoding_id, language_id and name_id
fn get_name_entry_string<'a>(font: &'a FontRef, platform_id: u16, encoding_id: u16, language_id: u16, name_id: StringId) -> Option<NameString<'a>> {
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


fn get_name_PEL_codes(name_table: Name) -> Option<Vec<Vec<(u16, u16, u16)>>> {
    let mut codes: HashMap<(u16, u16, u16), Vec<(u16, u16, u16)>> = HashMap::new();
    for rec in &name_table.name_record {
        let code = (rec.platform_id, rec.encoding_id, rec.language_id);
        codes.entry(code).or_default().push(code);
    }
    // Remove duplicates by converting to a HashSet
    let mut unique_codes: HashSet<(u16, u16, u16)> = HashSet::new();
    for code in codes.keys() {
        unique_codes.insert(*code);
    }
    // Convert HashSet back to Vec and sort it
    let mut codes_vec = vec![unique_codes.into_iter().collect::<Vec<(u16, u16, u16)>>()];
    codes_vec.iter_mut().for_each(|v| v.sort());
    Some(codes_vec)
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
    use serde_json::json;
    use fontspector_checkapi::{Context, Testable};

    #[test]
    fn test_get_name_ids_from_string() {
        let string_id = get_string_id_from_string("TYPOGRAPHIC_FAMILY_NAME");
        assert_eq!(string_id.unwrap(), StringId::TYPOGRAPHIC_FAMILY_NAME);
    }

    #[test]
    fn test_get_name_PEL_codes() {
        let mut tests: Vec<(Vec<(u16, u16, u16, u16, &str)>, Vec<(u16, u16, u16)>)> = Vec::new();
        tests.push(([(3, 1, 1033, 0, "Copyright"), (3, 1, 1033, 11, "https://fontwerk.com")].to_vec(), [(3, 1, 1033),].to_vec()));
        tests.push(([(1, 1, 1033, 0, "Copyright"), (3, 1, 1033, 0, "Copyright")].to_vec(), [ (1, 1, 1033), (3, 1, 1033)].to_vec()));
        tests.push(([].to_vec(), [].to_vec()));

        for (name_recs, expected_codes) in tests.iter() {
            let mut name_table = Name::default();
            let mut new_records = Vec::new();
            for (platform_id, encoding_id, language_id, name_id, name_string) in name_recs.iter() {
                let name_rec = NameRecord::new(
                    *platform_id,
                    *encoding_id,
                    *language_id,
                    NameId::new(*name_id),
                    String::from(*name_string).into(),
                );
                new_records.push(name_rec);
            }
            new_records.sort();
            name_table.name_record = new_records;
            let name_codes = get_name_PEL_codes(name_table);
            assert_eq!(name_codes, Some(vec![expected_codes.clone()]));
        }
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

        let configs = [config_1, config_2, config_3, config_4];
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
        ];
        let mut builder = FontBuilder::new();
        // We need to add a default maxp table, because otherwise
        // Testable::new_with_contents complains "Not a TTF file"
        builder.add_table(&Maxp::default()).unwrap();
        let mut name_table = Name::default();
        let mut new_records = Vec::new();
        let name_rec = NameRecord::new(3, 1, 1033, NameId::new(8), String::from("Fontwerk").into());
        new_records.push(name_rec);
        let name_rec = NameRecord::new(
            3,
            1,
            1033,
            NameId::new(11),
            String::from("https://fontwerk.com").into(),
        );
        new_records.push(name_rec);
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
        tests.push((StatusCode::Pass, None, [(1, "Family Name"), (2, "Bold"), (4, "Family Name Bold"), (16, "Family Name"), (17, "Bold")].to_vec()));
        tests.push((StatusCode::Fail, Some("The following issues have been found:\n\n* Missing required name table entry: FULL_NAME".to_string()), [(1, "Family Name"), (2, "Bold"), (16, "Family Name"), (17, "Bold")].to_vec()));
        tests.push((StatusCode::Fail, Some("The following issues have been found:\n\n* Inconsistent names: Family Name Bold (1 + 2) != Family Name Medium (16 + 17)".to_string()), [(1, "Family Name"), (2, "Bold"), (4, "Family Name Bold"), (16, "Family Name"), (17, "Medium")].to_vec()));
        tests.push((StatusCode::Fail, Some("The following issues have been found:\n\n* Missing required name table entry: TYPOGRAPHIC_SUBFAMILY_NAME".to_string()), [(1, "Family Name"), (2, "Bold"), (4, "Family Name Bold"), (16, "Family Name")].to_vec()));
        tests.push((StatusCode::Fail, Some("The following issues have been found:\n\n* Missing required name table entry: TYPOGRAPHIC_FAMILY_NAME".to_string()), [(1, "Family Name"), (2, "Bold"), (4, "Family Name Bold"), (17, "Bold")].to_vec()));
        tests.push((StatusCode::Fail, Some("The following issues have been found:\n\n* Missing required name table entry: SUBFAMILY_NAME".to_string()), [(1, "Family Name"), (4, "Family Name Bold"), (16, "Family Name"), (17, "Bold")].to_vec()));
        tests.push((StatusCode::Fail, Some("The following issues have been found:\n\n* Missing required name table entry: FAMILY_NAME".to_string()), [(2, "Bold"), (4, "Family Name Bold"), (16, "Family Name"), (17, "Bold")].to_vec()));
        tests.push((StatusCode::Fail, Some("The following issues have been found:\n\n* Inconsistent names: Family Name Condensed Bold (1 + 2) != Family Name Cond Bold (21 + 22)".to_string()), [(1, "Family Name Condensed"), (2, "Bold"), (4, "Family Name Condensed Bold"), (16, "Family Name Condensed"), (17, "Bold"), (21, "Family Name"), (22, "Cond Bold")].to_vec()));
        for (expected_severity, expected_message, records) in tests.iter(){
            let mut builder = FontBuilder::new();
            builder.add_table(&Maxp::default()).unwrap();
            let mut name_table = Name::default();
            let mut new_records = Vec::new();
            for rec in records.iter() {
                let name_rec = NameRecord::new(
                    3,
                    1,
                    1033,
                    NameId::new(rec.0),
                    String::from(rec.1).into(),
                );
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

}

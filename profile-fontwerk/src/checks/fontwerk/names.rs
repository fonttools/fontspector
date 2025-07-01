use fontations::skrifa::string::StringId;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use std::collections::HashMap;

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

    #[test]
    fn test_get_name_ids_from_string() {
        let string_id = get_string_id_from_string("TYPOGRAPHIC_FAMILY_NAME");
        assert_eq!(string_id.unwrap(), StringId::TYPOGRAPHIC_FAMILY_NAME);
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
}

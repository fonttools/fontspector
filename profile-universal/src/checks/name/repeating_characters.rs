use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;
use skrifa::raw::types::NameId;
use skrifa::raw::TableProvider;

#[check(
    id = "name/repeating_characters",
    rationale = "
        Duplicate spaces (or other non-visible characters) can be overlooked easily,
        which is why it's important to catch them.
        
        By default:
        FAIL: [<space>]
        WARN: [-]
        PASS: [a-z, A-Z, 0-9]

        name_ids_to_be_checked: [1, 2, 3, 4, 6, 16, 17, 21, 22, 25]
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/11",
    title = "Are there duplicate characters in the NAME table?"
)]
fn repeating_characters(t: &Testable, context: &Context) -> CheckFnResult {
    let config = context.local_config("name/repeating_characters");

    let default_name_ids_to_be_checked: Vec<u16> = vec![1, 2, 3, 4, 6, 16, 17, 21, 22, 25];
    let name_ids_to_be_checked = config
        .get("name_ids_to_be_checked")
        .map_or(default_name_ids_to_be_checked, |v| {
            convert_json_to_vec_numbers(v)
        });
    let check_name_ids = name_ids_to_be_checked
        .iter()
        .map(|v| NameId::new(*v))
        .collect::<Vec<_>>();

    let default_fail_chars = vec![' ']; // default space
    let fail_chars = config
        .get("FAIL")
        .map_or(default_fail_chars, |v| convert_json_to_vec_chars(v));

    let default_warn_chars = vec!['-'];
    let warn_chars = config
        .get("WARN")
        .map_or(default_warn_chars, |v| convert_json_to_vec_chars(v));

    let mut default_pass_chars = ('a'..='z').collect::<Vec<char>>();
    default_pass_chars.extend(('A'..='Z').collect::<Vec<char>>());
    default_pass_chars.extend(('0'..='9').collect::<Vec<char>>());
    let pass_chars: Vec<char> = config
        .get("PASS")
        .map_or(default_pass_chars, |v| convert_json_to_vec_chars(v));

    let mut problems: Vec<Status> = vec![];

    if let Ok(name_table) = testfont!(t).font().name() {
        for name_record in name_table.name_record().iter() {
            if !check_name_ids.contains(&name_record.name_id.get()) {
                // Skipping name record with name_id not in the list of IDs to be checked.
                // Often name IDs containing URLs have repeated characters eg. www
                continue;
            }

            let name_rec_info = format!(
                "{:}/{:}/{:}/{:}",
                name_record.platform_id,
                name_record.encoding_id,
                name_record.language_id,
                name_record.name_id,
            );

            let string_value = name_record
                .string(name_table.string_data())
                .map_err(|_| FontspectorError::General("Error reading string".to_string()))?
                .to_string();
            let repeated_chars = repeating_characters_in_a_row(&string_value);

            if !repeated_chars.is_empty() {
                for c in repeated_chars.iter() {
                    let message = format!(
                        "Name table record {name_rec_info} has repeated characters `{c}`:\n`{string_value}`",
                    );
                    let mut status = Status::info("repeated-characters-info", &message);
                    // if any repeated characters are in the fail list, mark as fail
                    if fail_chars.contains(c) {
                        status = Status::fail("repeated-characters-fail", &message);
                    } else if warn_chars.contains(c) {
                        status = Status::warn("repeated-characters-warn", &message);
                    } else if pass_chars.contains(c) {
                        // skip adding a status for repeated characters that are in the pass list
                        continue;
                    }
                    status.add_metadata(Metadata::TableProblem {
                        table_tag: "name".to_string(),
                        field_name: Some(format!("nameID {}", name_record.name_id)),
                        actual: Some(json!(string_value)),
                        expected: Some(json!(string_value
                            .chars()
                            .filter(|c| !repeated_chars.contains(c))
                            .collect::<String>())),
                        message,
                    });
                    problems.push(status);
                }
            }
        }
    }
    return_result(problems)
}

fn convert_json_to_vec_numbers(json_value: &serde_json::Value) -> Vec<u16> {
    if let Some(array) = json_value.as_array() {
        array
            .iter()
            .filter_map(|v| v.as_u64())
            .filter_map(|v| u16::try_from(v).ok())
            .collect()
    } else {
        Vec::new()
    }
}

fn convert_json_to_vec_chars(json_value: &serde_json::Value) -> Vec<char> {
    if let Some(array) = json_value.as_array() {
        array
            .iter()
            .filter_map(|v| v.as_str())
            .flat_map(|s| s.chars())
            .collect()
    } else {
        Vec::new()
    }
}

fn repeating_characters_in_a_row(s: &str) -> Vec<char> {
    let mut chars = s.chars();
    let mut repeated = Vec::new();

    let Some(mut previous) = chars.next() else {
        return repeated;
    };

    for c in chars {
        if c == previous {
            repeated.push(c);
        }
        previous = c;
    }

    repeated
}

#[cfg(test)]
mod tests {
    use super::repeating_characters;
    use std::collections::HashMap;

    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_pass, assert_results_contain, run_check,
            run_check_with_config, test_able,
        },
        StatusCode, Testable, TestableType,
    };
    use serde_json::json;
    use skrifa::raw::types::NameId;
    use write_fonts::{
        tables::{
            maxp::Maxp,
            name::{Name, NameRecord},
        },
        FontBuilder,
    };

    #[test]
    fn test_repeating_characters_pass() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let results = run_check(repeating_characters, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_repeating_characters_warn() {
        let mut builder = FontBuilder::new();
        builder.add_table(&Maxp::default()).unwrap();
        let mut name_table = Name::default();
        let mut new_records = Vec::new();
        let name_rec = NameRecord::new(
            3,
            1,
            1033,
            NameId::new(6),
            ("SomeFamilyname--Regular").to_string().into(),
        );
        new_records.push(name_rec);
        new_records.sort();
        name_table.name_record = new_records;
        builder.add_table(&name_table).unwrap();

        let testable = Testable::new_with_contents("demo.ttf", builder.build().clone());

        let results = run_check(repeating_characters, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("repeated-characters-warn".to_string()),
        );
        assert_messages_contain(
            &results,
            "Name table record 3/1/1033/POSTSCRIPT_NAME has repeated characters `-`",
        );
    }

    #[test]
    fn test_repeating_characters_fail() {
        let mut builder = FontBuilder::new();
        builder.add_table(&Maxp::default()).unwrap();
        let mut name_table = Name::default();
        let mut new_records = Vec::new();
        let name_rec = NameRecord::new(
            3,
            1,
            1033,
            NameId::new(1),
            ("Some  Familyname").to_string().into(),
        );
        new_records.push(name_rec);

        new_records.sort();
        name_table.name_record = new_records;
        builder.add_table(&name_table).unwrap();

        let testable = Testable::new_with_contents("demo.ttf", builder.build().clone());

        let results = run_check(repeating_characters, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("repeated-characters-fail".to_string()),
        );
        assert_messages_contain(
            &results,
            "Name table record 3/1/1033/FAMILY_NAME has repeated characters ` `",
        );
    }

    #[test]
    fn test_repeating_characters_fail_with_config() {
        let mut builder = FontBuilder::new();
        builder.add_table(&Maxp::default()).unwrap();
        let mut name_table = Name::default();
        let mut new_records = Vec::new();
        let name_rec = NameRecord::new(
            3,
            1,
            1033,
            NameId::new(1234),
            ("SomeFFamilyname").to_string().into(),
        );
        new_records.push(name_rec);

        new_records.sort();
        name_table.name_record = new_records;
        builder.add_table(&name_table).unwrap();

        let testable = Testable::new_with_contents("demo.ttf", builder.build().clone());

        let config = HashMap::from([(
            "name/repeating_characters".to_string(),
            json!({"FAIL": ["F"], "name_ids_to_be_checked": [1234]}),
        )]);
        let results = run_check_with_config(
            repeating_characters,
            TestableType::Single(&testable),
            config,
        );
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("repeated-characters-fail".to_string()),
        );
        assert_messages_contain(
            &results,
            "Name table record 3/1/1033/NameId 1234 has repeated characters `F`",
        );
    }
}

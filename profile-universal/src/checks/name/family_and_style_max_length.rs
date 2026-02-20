use std::collections::HashMap;

use fontations::skrifa::raw::{tables::name::Name, types::NameId, TableProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

fn strip_ribbi(name: &str) -> String {
    name.replace(" Regular", "")
        .replace(" Bold Italic", "")
        .replace(" Bold", "")
        .replace(" Italic", "")
}

// We are matching name entries with the same platform/language/encoding, so we
// need to use low-level APIS here
fn low_level_names(name: &Name<'_>, name_id: NameId) -> HashMap<(u16, u16, u16), String> {
    name.name_record()
        .iter()
        .filter(|r| r.name_id() == name_id)
        .map(|r| {
            (
                (r.platform_id(), r.encoding_id(), r.language_id()), // key
                r.string(name.string_data())
                    .map(|ns| ns.chars().collect::<String>())
                    .unwrap_or("".to_string()), // value
            )
        })
        .collect()
}

#[check(
    id = "name/family_and_style_max_length",
    rationale = "
        This check ensures that the length of name table entries is not
        too long, as this causes problems in some environments.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/1488",
    proposal = "https://github.com/fonttools/fontbakery/issues/2179",
    title = "Combined length of family and style must not exceed 32 characters."
)]
fn family_and_style_max_length(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    if !f.has_table(b"name") {
        return Ok(Status::just_one_fail("lacks-table", "No name table."));
    }
    let config = context.local_config("name/family_and_style_max_length");
    let full_name_length: usize = config
        .get("FULL_NAME")
        .and_then(|v| v.as_u64())
        .unwrap_or(32) as usize;

    let postscript_name_length: usize = config
        .get("POSTSCRIPT_NAME")
        .and_then(|v| v.as_u64())
        .unwrap_or(27) as usize;

    let instance_name_length: usize = config
        .get("INSTANCE_NAME")
        .and_then(|v| v.as_u64())
        .unwrap_or(32) as usize;

    let mut problems = vec![];
    for name in f.get_name_entry_strings(NameId::FULL_NAME) {
        if strip_ribbi(&name).len() > full_name_length {
            let chars_too_long_count = strip_ribbi(&name).len() - full_name_length;
            let chars_too_long = chars_too_long_count.to_string();
            let message = format!(
                "Name ID 4 'Full Font Name' exceeds {} characters ({} characters too long). This has been found to cause problems with the dropdown menu in old versions of Microsoft Word as well as shaping issues for some accented letters in Microsoft Word on Windows 10 and 11.",
                full_name_length,
                chars_too_long
            );
            let mut status = Status::fail("nameid4-too-long", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "name".to_string(),
                field_name: Some("nameID 4".to_string()),
                actual: Some(json!(strip_ribbi(&name).len())),
                expected: Some(json!(full_name_length)),
                message: message.clone(),
            });
            problems.push(status);
        }
    }
    for name in f.get_name_entry_strings(NameId::POSTSCRIPT_NAME) {
        if name.len() > postscript_name_length {
            let chars_too_long_count = name.len() - postscript_name_length;
            let chars_too_long = chars_too_long_count.to_string();
            let message = format!(
                "Name ID 6 'PostScript Name' exceeds {} characters ({} characters too long). This has been found to cause problems with PostScript printers, especially on Mac platforms.",
                postscript_name_length,
                chars_too_long
            );
            let mut status = Status::warn("nameid6-too-long", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "name".to_string(),
                field_name: Some("nameID 6".to_string()),
                actual: Some(json!(name.len())),
                expected: Some(json!(postscript_name_length)),
                message: message.clone(),
            });
            problems.push(status);
        }
    }
    let name = f.font().name()?;
    let typo_family_names: HashMap<(u16, u16, u16), String> =
        low_level_names(&name, NameId::TYPOGRAPHIC_FAMILY_NAME);
    let family_names: HashMap<(u16, u16, u16), String> =
        low_level_names(&name, NameId::FAMILY_NAME);

    if f.has_table(b"fvar") {
        for instance in f.font().fvar()?.instances()?.iter().flatten() {
            for instance_name in f.get_name_entry_strings(instance.subfamily_name_id) {
                for (key, string) in family_names.iter() {
                    // Use typo if present, nameid=1 otherwise
                    let family_name = typo_family_names.get(key).unwrap_or(string);
                    let full_instance_name = format!("{family_name} {instance_name}");
                    if full_instance_name.len() > instance_name_length {
                        let chars_too_long_count = full_instance_name.len() - instance_name_length;
                        let chars_too_long = chars_too_long_count.to_string();
                        let message = format!(
                            "Variable font instance name '{}' formed by space-separated concatenation of font family name (nameID {}) and instance subfamily nameID {} exceeds {} characterss ({} characters too long).\n\nThis has been found to cause shaping issues for some accented letters in Microsoft Word on Windows 10 and 11.",
                            full_instance_name,
                            NameId::FAMILY_NAME,
                            instance_name,
                            instance_name_length,
                            chars_too_long
                        );
                        let mut status = Status::fail("instance-too-long", &message);
                        status.add_metadata(Metadata::TableProblem {
                            table_tag: "fvar".to_string(),
                            field_name: Some("instance name".to_string()),
                            actual: Some(json!(full_instance_name.len())),
                            expected: Some(json!(instance_name_length)),
                            message: message.clone(),
                        });
                        problems.push(status);
                    }
                }
            }
        }
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_pass, assert_results_contain, run_check_with_config,
            test_able,
        },
        StatusCode, TestableType,
    };
    use std::collections::HashMap;

    #[test]
    fn test_family_and_style_max_length_fail_nid4() {
        let conf = HashMap::from([(
            "name/family_and_style_max_length".to_string(),
            serde_json::json!({ "FULL_NAME": 3}),
        )]);
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check_with_config(
            super::family_and_style_max_length,
            TestableType::Single(&testable),
            conf,
        );
        assert_messages_contain(&results, "(2 characters too long)");
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("nameid4-too-long".to_string()),
        );
    }

    #[test]
    fn test_family_and_style_max_length_fail_nid6() {
        let conf = HashMap::from([(
            "name/family_and_style_max_length".to_string(),
            serde_json::json!({ "POSTSCRIPT_NAME": 11}),
        )]);
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check_with_config(
            super::family_and_style_max_length,
            TestableType::Single(&testable),
            conf,
        );
        assert_messages_contain(&results, "(2 characters too long)");
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("nameid6-too-long".to_string()),
        );
    }

    #[test]
    fn test_family_and_style_max_length_fail_instance_name() {
        let conf = HashMap::from([(
            "name/family_and_style_max_length".to_string(),
            serde_json::json!({ "INSTANCE_NAME": 11}),
        )]);
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check_with_config(
            super::family_and_style_max_length,
            TestableType::Single(&testable),
            conf,
        );
        assert_messages_contain(&results, "(6 characters too long)");
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("instance-too-long".to_string()),
        );
    }

    #[test]
    fn test_family_and_style_max_length_pass_no_config() {
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check_with_config(
            super::family_and_style_max_length,
            TestableType::Single(&testable),
            HashMap::new(),
        );
        assert_pass(&results);
    }
}

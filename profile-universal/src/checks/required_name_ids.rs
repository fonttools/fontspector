use fontations::skrifa::string::StringId;
use fontspector_checkapi::{
    get_name_entry_string, get_name_platform_tuples, prelude::*, testfont, FileTypeConvert,
    Metadata, PlatformSelector,
};
use serde_json::json;

use std::vec;

#[check(
    id = "universal/required_name_ids",
    rationale = "
        Check required name ids based on given list.
    ",
    title = "Required name ids in name table"
)]
fn required_name_ids(t: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    if !font.has_table(b"name") {
        return Ok(Status::just_one_fail("lacks-table", "No name table."));
    }

    let config = context.local_config("universal/required_name_ids");
    let required_ids: Vec<u16> = config
        .get("required_name_ids")
        .ok_or(FontspectorError::skip(
            "no-required-name-ids",
            "Add the `required_name_ids` key to a `fontspector.toml` file.",
        ))?
        .as_array()
        .ok_or(FontspectorError::skip(
            "invalid-required-name-ids",
            "The `required_name_ids` key in the configuration file must be an array.",
        ))?
        .iter()
        .filter_map(|v| v.as_u64().map(|n| n as u16))
        .collect();

    let mut bad_names: Vec<String> = vec![];

    let platform_tuples = get_name_platform_tuples(font.font());
    for platform_tuple in platform_tuples {
        let mut missing_name_ids: Vec<_> = vec![];
        for id in required_ids.clone().into_iter() {
            let selector = PlatformSelector {
                platform_id: platform_tuple.0,
                encoding_id: platform_tuple.1,
                language_id: platform_tuple.2,
            };
            let name_id = StringId::from(id);
            if let Some(_name_string) = get_name_entry_string(&font.font(), selector, name_id) {
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
                "Missing required name IDs {missing_name_ids:?} for {platform_tuple:?}.",
            ));
        }
    }

    if bad_names.is_empty() {
        Ok(Status::just_one_pass())
    } else {
        let message = format!(
            "The following issues have been found:\n\n{}",
            bullet_list(context, bad_names.clone())
        );
        let mut status = Status::fail("missing-name-table-ids", &message);
        status.add_metadata(Metadata::FontProblem {
            message: message.clone(),
            context: Some(json!({
                "missing_name_ids_issues": bad_names,
                "total_issues": bad_names.len(),
            })),
        });
        return_result(vec![status])
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use fontspector_checkapi::StatusCode;
    use std::collections::HashMap;

    use fontspector_checkapi::codetesting::{
        assert_messages_contain, assert_pass, assert_results_contain, run_check_with_config,
        test_able,
    };

    #[test]
    fn test_required_name_ids_fail() {
        let conf = HashMap::from([(
            "universal/required_name_ids".to_string(),
            serde_json::json!({ "required_name_ids": [0, 1, 1234] }),
        )]);
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check_with_config(
            super::required_name_ids,
            TestableType::Single(&testable),
            conf,
        );
        assert_messages_contain(&results, "Missing required name IDs [1234]");
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("missing-name-table-ids".to_string()),
        );
    }

    #[test]
    fn test_required_name_ids_pass() {
        let conf = HashMap::from([(
            "universal/required_name_ids".to_string(),
            serde_json::json!({ "required_name_ids": [260] }), // name ID exists in all platforms
        )]);
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check_with_config(
            super::required_name_ids,
            TestableType::Single(&testable),
            conf,
        );
        assert_pass(&results);
    }
}

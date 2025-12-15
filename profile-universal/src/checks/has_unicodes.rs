use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "has_unicodes",
    rationale = "Some foundries want to know that a font contains certain unicodes.

    This check expects to find a table of unicodes in the configuration file, and checks to ensure that the font includes these unicodes.

    Example:

    ```
    has_unicodes = [0x0020, 0x0041, 0x1F60A, 0x1F680]
    ```

    Alternatively, the configuration can be specialized on a per-font basis:

    ```
    [has_unicodes]
    \"Foo-Regular.ttf\" = [0x0020, 0x0041, 0x1F60A, 0x1F680]
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/526",
    title = "Check required unicodes"
)]
fn has_unicodes(t: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let codepoints = font.codepoints(Some(context));
    let config = context.local_config("has_unicodes");
    skip!(
        config.is_null(),
        "unconfigured",
        "No configuration found for has_unicodes"
    );
    let font_config = if config.is_object() {
        let font_name = t.basename().unwrap_or("<Unnamed Font>".to_string());
        // If the config is a table of tables, specialize it by font filename
        if let Some(specific) = config.as_object().and_then(|o| o.get(&font_name)) {
            specific
        } else {
            skip!(
                "unconfigured",
                &format!("No specific configuration found for {}", font_name)
            );
        }
    } else {
        &config
    };

    if let Some(config_for_this_font) = font_config.as_array() {
        let mut problems = vec![];
        let required_unicodes: Vec<u32> = config_for_this_font
            .iter()
            .filter_map(|v| v.as_u64().map(|n| n as u32))
            .collect();

        // Check encoded glyphs
        let missing = required_unicodes
            .iter()
            .filter(|&&c| !codepoints.contains(&c))
            .cloned()
            .map(|c| format!("uni{c:04X}"))
            .collect::<Vec<String>>();

        if !missing.is_empty() {
            problems.push(Status::fail(
                "missing-unicodes",
                &format!("Font is missing required unicodes: {}", missing.join(", ")),
            ));
        }
        return_result(problems)
    } else {
        return Err(FontspectorError::General(
            "Configuration for has_unicodes is not an object or a list".to_string(),
        ));
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use fontspector_checkapi::StatusCode;
    use serde_json::json;
    use std::collections::HashMap;

    use fontspector_checkapi::codetesting::{
        assert_messages_contain, assert_results_contain, run_check_with_config, test_able,
    };

    #[test]
    fn test_has_unicodes() {
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let font_name = testable.basename().unwrap_or("<Unnamed Font>".to_string());
        let conf = HashMap::from([(
            font_name,
            serde_json::json!(json!([0x0020, 0x0041, 0x1F60A, 0x1F680])),
        )]);
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results =
            run_check_with_config(super::has_unicodes, TestableType::Single(&testable), conf);
        assert_messages_contain(
            &results,
            "Font is missing required unicodes: uni1F60A, uni1F680",
        );
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("missing-unicodes".to_string()),
        );
    }
}

use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

// TODO: maybe move this to universal and
// replace fontwerk/embedding_bit and other similar checks

#[check(
    id = "monotype/fstype",
    rationale = "
        According to Monotype the value of the OS/2.fstype (also known as 'Embedding bit') field must be:
        1. for retail: Print & Preview (Bit 4).
        2. for custom (in most cases): Editable Embedding (Bit 8).
    ",
    title = "Checking embedding bit (OS/2 fsType)."
)]
fn fstype(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let config = context.local_config("monotype/fstype");
    let expected_fstype_value = config.get("fstype_value")
        .ok_or(FontspectorError::skip(
            "no-fstype-value",
            "Add the `fstype_value` key to a `fontspector.toml` file on your font project directory to enable this check.\nYou'll also need to use the `--configuration` flag when invoking fontspector",
        ))?
        .as_u64()
        .ok_or(FontspectorError::skip(
            "invalid-fstype-value",
            "The `fstype_value` key in the configuration file must be an integer.",
        ))? as u16;
    let fstype_val = f.font().os2()?.fs_type();
    Ok(if fstype_val == expected_fstype_value {
        Status::just_one_pass()
    } else {
        let expected_embedding = fs_type_val_name(expected_fstype_value);
        let found_embedding = fs_type_val_name(fstype_val);
        Status::just_one_fail(
            "fstype",
            &format!(
                "OS/2 fsType must be set to {expected_embedding}, found {found_embedding} instead."
            ),
        )
    })
}

fn fs_type_val_name(expected_fstype_value: u16) -> &'static str {
    match expected_fstype_value {
        0 => "Installable Embedding (0)",
        2 => "Restricted License Embedding (2)",
        4 => "Print & Preview (4)",
        8 => "Editable Embedding (8)",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_pass, assert_results_contain, run_check_with_config,
            test_able,
        },
        StatusCode, TestableType,
    };
    use serde_json::json;

    #[test]
    fn test_fstype_pass() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let config = HashMap::from([("monotype/fstype".to_string(), json!({"fstype_value": 0}))]);
        let results = run_check_with_config(super::fstype, TestableType::Single(&testable), config);
        assert_pass(&results);
    }

    #[test]
    fn test_fstype_fail() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let config = HashMap::from([("monotype/fstype".to_string(), json!({"fstype_value": 4}))]);
        let results = run_check_with_config(super::fstype, TestableType::Single(&testable), config);
        assert_results_contain(&results, StatusCode::Fail, Some("fstype".to_string()));
        assert_messages_contain(
            &results,
            "OS/2 fsType must be set to Print & Preview (4), found Installable Embedding (0) instead.",
        );
    }
}

use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "has_features",
    rationale = "Some foundries want to know that a font contains certain OpenType layout features.

    This check expects to find a table of feature names in the configuration file, and checks to ensure that the font includes these features.

    Example:

    ```
    has_features = [ \"kern\", \"ss01\", \"calt\" ]
    ```

    Alternatively, the configuration can be specialized on a per-font basis:

    ```
    [has_features]
    \"Foo-Regular.ttf\" = [ \"kern\", \"ss01\", \"calt\" ]
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/406",
    title = "Ensure OpenType features are present."
)]
fn has_features(t: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let config = context.local_config("has_features");
    skip!(
        config.is_null(),
        "unconfigured",
        "No configuration found for has_features"
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
        for feature in config_for_this_font {
            if let Some(feature) = feature.as_str() {
                if !font.has_feature(false, feature) {
                    let message = format!("Font is missing required feature {}", feature);
                    let mut status = Status::fail("missing-feature", &message);
                    status.add_metadata(Metadata::FontProblem {
                        message: message.clone(),
                        context: Some(json!({ "required_feature": feature })),
                    });
                    problems.push(status);
                }
            }
        }
        return_result(problems)
    } else {
        return Err(FontspectorError::General(
            "Configuration for has_features is not an object or a list".to_string(),
        ));
    }
}

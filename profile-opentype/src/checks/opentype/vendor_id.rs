use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/vendor_id",
    rationale = "
        When a font project's Vendor ID is specified explicitly on FontBakery's
        configuration file, all binaries must have a matching vendor identifier
        value in the OS/2 table.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3941",
    title = "Check OS/2 achVendID against configuration"
)]
fn vendor_id(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let config = context.local_config("opentype/vendor_id");
    let expected_vendor_id = config.get("vendor_id")
        .ok_or(FontspectorError::skip(
            "no-vendor-id",
            "Add the `vendor_id` key to a `fontspector.toml` file on your font project directory to enable this check.\nYou'll also need to use the `--configuration` flag when invoking fontspector",
        ))?
        .as_str()
        .ok_or(FontspectorError::skip(
            "invalid-vendor-id",
            "The `vendor_id` key in the configuration file must be a string.",
        ))?;
    let os2_vendor_id = font.font().os2()?.ach_vend_id().to_string();
    let mut problems = vec![];
    if os2_vendor_id.as_str() != expected_vendor_id {
        let msg = format!(
            "OS/2 achVendID value '{os2_vendor_id}' does not match configuration value '{expected_vendor_id}'"
        );
        let mut status = Status::fail("bad-vendor-id", &msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("achVendID".to_string()),
            actual: Some(json!(os2_vendor_id.clone())),
            expected: Some(json!(expected_vendor_id.to_string())),
            message: msg,
        });
        problems.push(status);
    }
    return_result(problems)
}

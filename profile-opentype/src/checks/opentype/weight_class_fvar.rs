use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/weight_class_fvar",
    rationale = "According to Microsoft's OT Spec the OS/2 usWeightClass should match the fvar default value.",
    proposal = "https://github.com/googlefonts/gftools/issues/477",
    title = "Checking if OS/2 usWeightClass matches fvar."
)]
fn weight_class_fvar(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(!f.is_variable_font(), "not-variable", "Not a variable font");
    let fvar_value = f
        .axis_ranges()
        .find(|(tag, _, _, _)| tag == "wght")
        .map(|(_, _, default, _)| default)
        .ok_or(FontspectorError::skip("no-wght", "No 'wght' axis"))?;
    let os2_value = f
        .font()
        .os2()
        .map_err(|_| FontspectorError::skip("no-os2", "No OS/2 table"))?
        .us_weight_class();
    let mut problems = vec![];
    if os2_value != fvar_value as u16 {
        let msg = format!("OS/2 usWeightClass is {os2_value}, but fvar default is {fvar_value}");
        let mut status = Status::fail("bad-weight-class", &msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("usWeightClass".to_string()),
            actual: Some(json!(os2_value)),
            expected: Some(json!(fvar_value)),
            message: msg,
        });
        problems.push(status);
    }
    return_result(problems)
}

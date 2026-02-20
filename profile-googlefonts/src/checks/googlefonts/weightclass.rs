use fontations::skrifa::{raw::TableProvider, FontRef};
use fontspector_checkapi::{
    constants::OutlineType, prelude::*, testfont, FileTypeConvert, Metadata,
};
use serde_json::json;

use crate::utils::build_expected_font;

#[check(
    id = "googlefonts/weightclass",
    rationale = "
        
        Google Fonts expects variable fonts, static ttfs and static otfs to have
        differing OS/2 usWeightClass values.

        - For Variable Fonts, Thin-Black must be 100-900

        - For static ttfs, Thin-Black can be 100-900 or 250-900

        - For static otfs, Thin-Black must be 250-900

        If static otfs are set lower than 250, text may appear blurry in
        legacy Windows applications.

        Glyphsapp users can change the usWeightClass value of an instance by adding
        a 'weightClass' customParameter.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Check the OS/2 usWeightClass is appropriate for the font's best SubFamily name."
)]
fn weightclass(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let value = f.font().os2()?.us_weight_class();
    let expected_names = build_expected_font(&f, &[])?;
    let expected_value = FontRef::new(&expected_names)?.os2()?.us_weight_class();
    let style_name = f.best_subfamilyname().unwrap_or("Regular".to_string());
    if f.is_variable_font() {
        if value != expected_value {
            let msg = "OS/2 usWeightClass does not match expected value for variable font";
            let mut status = Status::fail(
                "bad-value",
                &format!(
                    "Best SubFamily name is '{style_name}'. Expected OS/2 usWeightClass is {expected_value}, got {value}."
                ),
            );
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("usWeightClass".to_string()),
                actual: Some(json!(value)),
                expected: Some(json!(expected_value)),
                message: msg.to_string(),
            });
            problems.push(status);
        }
    } else if style_name.contains("Thin") {
        if f.outline_type() == OutlineType::TrueType && ![100, 250].contains(&value) {
            let msg = "OS/2 usWeightClass is invalid for Thin weight TrueType";
            let mut status = Status::fail(
                "bad-value",
                &format!(
                    "Best SubFamily name is '{style_name}'. Expected OS/2 usWeightClass is {expected_value}, got {value}."
                ),
            );
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("usWeightClass".to_string()),
                actual: Some(json!(value)),
                expected: Some(json!([100, 250])),
                message: msg.to_string(),
            });
            problems.push(status);
        }
        if f.outline_type() == OutlineType::CFF && value != 250 {
            let msg = "OS/2 usWeightClass is invalid for Thin weight CFF";
            let mut status = Status::fail(
                "bad-value",
                &format!(
                    "Best SubFamily name is '{}'. Expected OS/2 usWeightClass is {}, got {}.",
                    style_name, 250, value
                ),
            );
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("usWeightClass".to_string()),
                actual: Some(json!(value)),
                expected: Some(json!(250)),
                message: msg.to_string(),
            });
            problems.push(status);
        }
    } else if style_name.contains("ExtraLight") {
        if f.outline_type() == OutlineType::TrueType && ![200, 275].contains(&value) {
            let msg = "OS/2 usWeightClass is invalid for ExtraLight weight TrueType";
            let mut status = Status::fail(
                "bad-value",
                &format!(
                    "Best SubFamily name is '{style_name}'. Expected OS/2 usWeightClass is {expected_value}, got {value}."
                ),
            );
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("usWeightClass".to_string()),
                actual: Some(json!(value)),
                expected: Some(json!([200, 275])),
                message: msg.to_string(),
            });
            problems.push(status);
        }
        if f.outline_type() == OutlineType::CFF && value != 275 {
            let msg = "OS/2 usWeightClass is invalid for ExtraLight weight CFF";
            let mut status = Status::fail(
                "bad-value",
                &format!(
                    "Best SubFamily name is '{}'. Expected OS/2 usWeightClass is {}, got {}.",
                    style_name, 275, value
                ),
            );
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("usWeightClass".to_string()),
                actual: Some(json!(value)),
                expected: Some(json!(275)),
                message: msg.to_string(),
            });
            problems.push(status);
        }
    } else if value != expected_value {
        let msg = "OS/2 usWeightClass does not match subfamily name";
        let mut status = Status::fail(
            "bad-value",
            &format!(
                "Best SubFamily name is '{style_name}'. Expected OS/2 usWeightClass is {expected_value}, got {value}."
            ),
        );
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("usWeightClass".to_string()),
            actual: Some(json!(value)),
            expected: Some(json!(expected_value)),
            message: msg.to_string(),
        });
        problems.push(status);
    }
    return_result(problems)
}

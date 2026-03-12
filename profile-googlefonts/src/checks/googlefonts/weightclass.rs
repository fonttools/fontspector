use fontations::{
    skrifa::{raw::TableProvider, FontRef},
    write::from_obj::ToOwnedTable,
};
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
    title = "Check the OS/2 usWeightClass is appropriate for the font's best SubFamily name.",
    hotfix = fix_weightclass,
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    use super::weightclass;

    #[test]
    fn test_fail_mada_regular() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(weightclass, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("bad-value".to_string()));
    }

    #[test]
    fn test_pass_cabin_fonts() {
        for font in [
            "cabin/Cabin-BoldItalic.ttf",
            "cabin/Cabin-Bold.ttf",
            "cabin/Cabin-Italic.ttf",
            "cabin/Cabin-MediumItalic.ttf",
            "cabin/Cabin-Medium.ttf",
            "cabin/Cabin-Regular.ttf",
            "cabin/Cabin-SemiBoldItalic.ttf",
            "cabin/Cabin-SemiBold.ttf",
        ] {
            let testable = test_able(font);
            let results = run_check(weightclass, testable);
            assert_pass(&results);
        }
    }
}

fn fix_weightclass(t: &mut Testable, _replies: Option<MoreInfoReplies>) -> Result<FixResult, FontspectorError> {
    let f = testfont!(t);
    let expected_names = build_expected_font(&f, &[])?;
    let expected_value = FontRef::new(&expected_names)?.os2()?.us_weight_class();
    let mut os2: fontations::write::tables::os2::Os2 = f.font().os2()?.to_owned_table();
    os2.us_weight_class = expected_value;
    t.set(f.rebuild_with_new_table(&os2)?);
    Ok(FixResult::Fixed)
}

use fontations::{skrifa::raw::TableProvider, write::from_obj::ToOwnedTable};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/weight_class_fvar",
    rationale = "According to Microsoft's OT Spec the OS/2 usWeightClass should match the fvar default value.",
    proposal = "https://github.com/googlefonts/gftools/issues/477",
    title = "Checking if OS/2 usWeightClass matches fvar.",
    hotfix = fix_weight_class_fvar,
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

fn fix_weight_class_fvar(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    if !f.is_variable_font() {
        return Ok(false);
    }
    let Some(fvar_value) = f
        .axis_ranges()
        .find(|(tag, _, _, _)| tag == "wght")
        .map(|(_, _, default, _)| default)
    else {
        return Ok(false);
    };
    let mut os2: fontations::write::tables::os2::Os2 = f.font().os2()?.to_owned_table();
    os2.us_weight_class = fvar_value as u16;
    t.set(f.rebuild_with_new_table(&os2)?);
    Ok(true)
}

#[cfg(test)]
mod tests {
    use fontations::{skrifa::raw::TableProvider, write::from_obj::ToOwnedTable};
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, assert_skip, run_check, test_able},
        prelude::*,
        FileTypeConvert, StatusCode,
    };

    #[test]
    fn test_weight_class_fvar_pass() {
        let testable = test_able("varfont/OpenSans[wdth,wght].ttf");
        let result = run_check(super::weight_class_fvar, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_weight_class_fvar_mismatch() {
        let mut testable = test_able("varfont/OpenSans[wdth,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut os2: fontations::write::tables::os2::Os2 = f.font().os2().unwrap().to_owned_table();
        os2.us_weight_class = 333;
        testable.set(f.rebuild_with_new_table(&os2).unwrap());
        let result = run_check(super::weight_class_fvar, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("bad-weight-class".to_string()),
        );
    }

    #[test]
    fn test_weight_class_fvar_no_wght_skip() {
        let testable = test_able("BadGrades/BadGrades-VF.ttf");
        let result = run_check(super::weight_class_fvar, testable);
        assert_skip(&result);
    }
}

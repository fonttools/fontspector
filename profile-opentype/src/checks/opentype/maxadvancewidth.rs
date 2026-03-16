use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/maxadvancewidth",
    title = "MaxAdvanceWidth is consistent with values in the Hmtx and Hhea tables?",
    rationale = "
        The 'hhea' table contains a field which specifies the maximum advance width.
        This value should be consistent with the maximum advance width of all glyphs
        specified in the 'hmtx' table.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
)]
fn maxadvancewidth(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let hhea_advance_width_max = f.font().hhea()?.advance_width_max().to_u16();
    let hmtx_advance_width_max = f
        .font()
        .hmtx()?
        .h_metrics()
        .iter()
        .map(|m| m.advance.get())
        .max()
        .unwrap_or_default();
    let mut problems = vec![];
    if hmtx_advance_width_max != hhea_advance_width_max {
        let msg = format!(
            "AdvanceWidthMax mismatch: expected {hmtx_advance_width_max} from hmtx; got {hhea_advance_width_max} for hhea"
        );
        let mut status = Status::fail("mismatch", &msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "hhea".to_string(),
            field_name: Some("advanceWidthMax".to_string()),
            actual: Some(json!(hhea_advance_width_max)),
            expected: Some(json!(hmtx_advance_width_max)),
            message: msg,
        });
        problems.push(status);
    }
    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use fontations::{skrifa::raw::TableProvider, write::from_obj::ToOwnedTable};
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        prelude::*,
        FileTypeConvert, StatusCode,
    };

    #[test]
    fn test_maxadvancewidth_pass() {
        let testable = test_able("familysans/FamilySans-Regular.ttf");
        let result = run_check(super::maxadvancewidth, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_maxadvancewidth_mismatch() {
        let mut testable = test_able("familysans/FamilySans-Regular.ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut hhea: fontations::write::tables::hhea::Hhea =
            f.font().hhea().unwrap().to_owned_table();
        hhea.advance_width_max = 32767u16.into();
        testable.set(f.rebuild_with_new_table(&hhea).unwrap());
        let result = run_check(super::maxadvancewidth, testable);
        assert_results_contain(&result, StatusCode::Fail, Some("mismatch".to_string()));
    }
}

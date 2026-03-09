use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/unitsperem",
    title = "Checking unitsPerEm value is reasonable.",
    rationale = "
        According to the OpenType spec:

        The value of unitsPerEm at the head table must be a value
        between 16 and 16384. Any value in this range is valid.

        In fonts that have TrueType outlines, a power of 2 is recommended
        as this allows performance optimizations in some rasterizers.

        But 1000 is a commonly used value. And 2000 may become
        increasingly more common on Variable Fonts.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
)]
fn unitsperem(f: &Testable, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    match testfont!(f).font().head()?.units_per_em() {
        bad_upem if !(16..=16384).contains(&bad_upem) => {
            let msg = format!(
                "unitsPerEm value must be a value between 16 and 16384. {bad_upem} is out of range"
            );
            let mut status = Status::fail("out-of-range", &msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "head".to_string(),
                field_name: Some("unitsPerEm".to_string()),
                actual: Some(json!(bad_upem)),
                expected: Some(json!("16-16384")),
                message: msg,
            });
            problems.push(status);
        }
        16 | 32 | 64 | 128 | 256 | 512 | 1024 | 2048 | 4096 | 8192 | 16384 | 1000 | 2000 => {
            // Valid values
        }
        upem => {
            let msg = format!("In order to optimize performance on some legacy renderers, the value of unitsPerEm at the head table should ideally be a power of 2 between 16 to 16384. And values of 1000 and 2000 are also common and may be just fine as well. But we got {upem} instead.");
            let mut status = Status::warn("suboptimal", &msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "head".to_string(),
                field_name: Some("unitsPerEm".to_string()),
                actual: Some(json!(upem)),
                expected: Some(json!("power of 2 or 1000/2000")),
                message: msg,
            });
            problems.push(status);
        }
    }
    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontations::{skrifa::raw::TableProvider, write::from_obj::ToOwnedTable};
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_unitsperem_pass() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let result = run_check(unitsperem, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_unitsperem_warn_suboptimal() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut head: fontations::write::tables::head::Head =
            f.font().head().unwrap().to_owned_table();
        head.units_per_em = 100;
        testable.set(f.rebuild_with_new_table(&head).unwrap());
        let result = run_check(unitsperem, testable);
        assert_results_contain(&result, StatusCode::Warn, Some("suboptimal".to_string()));
    }

    #[test]
    fn test_unitsperem_fail_out_of_range() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut head: fontations::write::tables::head::Head =
            f.font().head().unwrap().to_owned_table();
        head.units_per_em = 0;
        testable.set(f.rebuild_with_new_table(&head).unwrap());
        let result = run_check(unitsperem, testable);
        assert_results_contain(&result, StatusCode::Fail, Some("out-of-range".to_string()));
    }
}

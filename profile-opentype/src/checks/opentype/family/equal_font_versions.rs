use fontations::skrifa::raw::{ReadError, TableProvider};
use fontspector_checkapi::{prelude::*, FileTypeConvert, StatusCode};

#[check(
    id = "opentype/family/equal_font_versions",
    title = "Make sure all font files have the same version value.",
    rationale = "Within a family released at the same time, all members of the family should have the same version number in the head table.",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    implementation = "all"
)]
fn equal_font_versions(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(c);
    let versions_names: Result<Vec<_>, ReadError> = fonts
        .iter()
        .map(|f| {
            f.font().head().map(|head| {
                (
                    head.font_revision(),
                    format!("{:.03}", head.font_revision().to_f32()),
                    f.filename.to_string_lossy(),
                )
            })
        })
        .collect();
    assert_all_the_same(
        context,
        &versions_names?,
        "mismatch",
        "Version info differs among font files of the same font project.",
        StatusCode::Warn,
    )
}

#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontations::{skrifa::raw::TableProvider, write::from_obj::ToOwnedTable};
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check_with_config, test_able},
        StatusCode, TestableCollection, TestableType,
    };
    use std::collections::HashMap;

    #[test]
    fn test_equal_font_versions_pass() {
        let testables: Vec<_> = [
            "mada/Mada-Black.ttf",
            "mada/Mada-ExtraLight.ttf",
            "mada/Mada-Medium.ttf",
            "mada/Mada-SemiBold.ttf",
            "mada/Mada-Bold.ttf",
            "mada/Mada-Light.ttf",
            "mada/Mada-Regular.ttf",
        ]
        .iter()
        .map(test_able)
        .collect();
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let result = run_check_with_config(
            equal_font_versions,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_pass(&result);
    }

    #[test]
    fn test_equal_font_versions_warn_mismatch() {
        let mut testables: Vec<_> = [
            "mada/Mada-Black.ttf",
            "mada/Mada-ExtraLight.ttf",
            "mada/Mada-Medium.ttf",
            "mada/Mada-SemiBold.ttf",
            "mada/Mada-Bold.ttf",
            "mada/Mada-Light.ttf",
            "mada/Mada-Regular.ttf",
        ]
        .iter()
        .map(test_able)
        .collect();
        // Modify the second font's version
        let new_bytes = {
            let f = TTF.from_testable(&testables[1]).unwrap();
            let mut head: fontations::write::tables::head::Head =
                f.font().head().unwrap().to_owned_table();
            head.font_revision = fontations::skrifa::raw::types::Fixed::from_f64(99.0);
            f.rebuild_with_new_table(&head).unwrap()
        };
        testables[1].set(new_bytes);
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let result = run_check_with_config(
            equal_font_versions,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(&result, StatusCode::Warn, Some("mismatch".to_string()));
    }
}

use std::path::PathBuf;

use fontspector_checkapi::{prelude::*, skip, FileTypeConvert};
use itertools::Itertools;
use skrifa::raw::{types::FWord, TableProvider};

#[check(
    id = "opentype/family/underline_thickness",
    title = "Fonts have consistent underline thickness?",
    rationale = r#"
        Dave C Lemon (Adobe Type Team) recommends setting the underline thickness to be
        consistent across the family.

        If thicknesses are not family consistent, words set on the same line which have
        different styles look strange.
    "#,
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    implementation = "all"
)]
fn underline_thickness(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(c);
    skip!(fonts.len() < 2, "no-siblings", "No sibling fonts found");
    let posts: Vec<(&PathBuf, FWord)> = fonts
        .iter()
        .map(|font| {
            (
                &font.filename,
                font.font()
                    .post()
                    .map(|post| post.underline_thickness())
                    .unwrap_or_default(),
            )
        })
        .collect();
    Ok(if posts.iter().unique_by(|(_, t)| t).count() == 1 {
        Status::just_one_pass()
    } else {
        let mut message =
            "Underline thickness is inconsistent. Detected underline thickness values are:\n\n"
                .to_string();
        for (path, thickness) in posts {
            message.push_str(&format!("* {}: {}\n", path.display(), thickness));
        }
        Status::just_one_fail("inconsistent-underline-thickness", &message)
    })
}

#[allow(clippy::expect_used, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check_with_config, test_able},
        prelude::*,
        FileTypeConvert, StatusCode, TestableType,
    };
    use skrifa::raw::TableProvider;
    use std::collections::HashMap;
    use write_fonts::from_obj::ToOwnedTable;

    #[test]
    fn test_underline_thickness_pass() {
        let testables: Vec<Testable> = vec![
            test_able("mada/Mada-Black.ttf"),
            test_able("mada/Mada-ExtraLight.ttf"),
            test_able("mada/Mada-Medium.ttf"),
            test_able("mada/Mada-SemiBold.ttf"),
            test_able("mada/Mada-Bold.ttf"),
            test_able("mada/Mada-Light.ttf"),
            test_able("mada/Mada-Regular.ttf"),
        ];
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let result = run_check_with_config(
            super::underline_thickness,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_pass(&result);
    }

    #[test]
    fn test_underline_thickness_inconsistent() {
        let mut mada_black = test_able("mada/Mada-Black.ttf");
        let f = TTF.from_testable(&mada_black).unwrap();
        let mut post: write_fonts::tables::post::Post = f.font().post().unwrap().to_owned_table();
        let original = post.underline_thickness;
        post.underline_thickness = skrifa::raw::types::FWord::new(original.to_i16() + 1);
        mada_black.set(f.rebuild_with_new_table(&post).unwrap());
        let testables: Vec<Testable> = vec![mada_black, test_able("mada/Mada-Regular.ttf")];
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let result = run_check_with_config(
            super::underline_thickness,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("inconsistent-underline-thickness".to_string()),
        );
    }
}

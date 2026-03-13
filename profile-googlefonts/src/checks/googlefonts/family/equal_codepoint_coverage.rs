use itertools::Itertools;
use std::collections::HashSet;

use fontspector_checkapi::{prelude::*, skip, FileTypeConvert};

#[check(
    id = "googlefonts/family/equal_codepoint_coverage",
    title = "Fonts have equal codepoint coverage?",
    rationale = "For a given family, all fonts must have the same codepoint coverage.
                This is because we want to avoid the situation where, for example,
                a character is present in a regular font but missing in the italic
                style; turning on italic would cause the character to be rendered
                either as a fake italic (auto-slanted) or to show tofu.",
    proposal = "https://github.com/fonttools/fontbakery/issues/4180",
    implementation = "all"
)]
fn equal_codepoint_coverage(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(c);
    skip!(
        fonts.len() < 2,
        "no-siblings",
        "This check requires at least two sibling fonts to compare codepoint coverage."
    );
    let mut problems = vec![];
    let mut we_have_they_dont: HashSet<u32> = HashSet::new();
    let mut they_have_we_dont: HashSet<u32> = HashSet::new();
    #[allow(clippy::unwrap_used)] // We checked the length above
    let my_codepoints = fonts.first().unwrap().codepoints(Some(context));
    let siblings = fonts.iter().skip(1);
    for sibling in siblings {
        let their_codepoints = sibling.codepoints(None);
        we_have_they_dont.extend(my_codepoints.difference(&their_codepoints));
        they_have_we_dont.extend(their_codepoints.difference(&my_codepoints));
    }

    #[allow(clippy::unwrap_used)] // We checked the length above
    let name_of_first = fonts.first().unwrap().filename.to_str().unwrap(); // That's a lot of unwrap

    if !we_have_they_dont.is_empty() {
        problems.push(Status::fail(
            "glyphset-diverges",
            &format!(
                "Font {} has codepoints not present in sibling fonts: {}",
                name_of_first,
                we_have_they_dont
                    .iter()
                    .map(|i| format!("U+{i:04X}"))
                    .join(", ")
            ),
        ))
    }
    if !they_have_we_dont.is_empty() {
        problems.push(Status::fail(
            "glyphset-diverges",
            &format!(
                "Other fonts have codepoints not present in {}: {}",
                name_of_first,
                they_have_we_dont
                    .iter()
                    .map(|i| format!("U+{i:04X}"))
                    .join(", ")
            ),
        ))
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::equal_codepoint_coverage;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, deencode_glyph, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        fontspector_checkapi::codetesting::run_check_with_config(
            equal_codepoint_coverage,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    fn cabin_family() -> Vec<Testable> {
        vec![
            test_able("cabin/Cabin-BoldItalic.ttf"),
            test_able("cabin/Cabin-Bold.ttf"),
            test_able("cabin/Cabin-Italic.ttf"),
            test_able("cabin/Cabin-MediumItalic.ttf"),
            test_able("cabin/Cabin-Medium.ttf"),
            test_able("cabin/Cabin-Regular.ttf"),
            test_able("cabin/Cabin-SemiBoldItalic.ttf"),
            test_able("cabin/Cabin-SemiBold.ttf"),
        ]
    }

    #[test]
    fn test_check_family_equal_codepoint_coverage() {
        assert_pass(&run(cabin_family()));

        let mut bad = cabin_family();
        let to_modify = bad
            .get_mut(1)
            .unwrap_or_else(|| panic!("Expected at least two fonts in cabin family fixture"));
        if let Err(error) = deencode_glyph(to_modify, 8730) {
            panic!("Failed to deencode glyph for test setup: {error}");
        }
        assert_results_contain(
            &run(bad),
            StatusCode::Fail,
            Some("glyphset-diverges".to_string()),
        );
    }
}

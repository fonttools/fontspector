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

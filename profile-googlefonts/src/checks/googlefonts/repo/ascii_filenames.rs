use fontspector_checkapi::{prelude::*, Metadata};
use serde_json::json;

#[check(
    id = "googlefonts/repo/ascii_filenames",
    rationale = "
        The google/fonts repository should only contain files with ASCII
        filenames. Non-ASCII characters in filenames can cause issues
        across different operating systems and tools.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/167",
    title = "Ensure all filenames are ASCII-only.",
    implementation = "all"
)]
fn ascii_filenames(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    for testable in c.iter() {
        if let Some(name) = testable.basename() {
            if !name.is_ascii() {
                let message = format!("Non-ASCII filename: '{name}'");
                let mut status = Status::fail("non-ascii-filename", &message);
                status.add_metadata(Metadata::FontProblem {
                    message,
                    context: Some(json!({
                        "filename": name,
                    })),
                });
                problems.push(status);
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check_with_config},
        StatusCode, Testable, TestableCollection, TestableType,
    };
    use std::collections::HashMap;

    use super::ascii_filenames;

    fn make_collection(filenames: &[&str]) -> TestableCollection {
        let testables: Vec<Testable> = filenames
            .iter()
            .map(|name| Testable::new_with_contents(name, vec![]))
            .collect();
        TestableCollection::from_testables(testables, None)
    }

    #[test]
    fn test_ascii_filenames_pass() {
        let collection = make_collection(&[
            "ofl/notosans/NotoSans-Regular.ttf",
            "ofl/notosans/METADATA.pb",
            "ofl/notosans/OFL.txt",
        ]);
        let results = run_check_with_config(
            ascii_filenames,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_pass(&results);
    }

    #[test]
    fn test_non_ascii_filename_fails() {
        let collection =
            make_collection(&["ofl/notosans/NotoSans-Regular.ttf", "ofl/notosans/café.ttf"]);
        let results = run_check_with_config(
            ascii_filenames,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("non-ascii-filename".to_string()),
        );
    }

    #[test]
    fn test_non_ascii_metadata_filename_fails() {
        let collection = make_collection(&[
            "ofl/notosans/NotoSans-Regular.ttf",
            "ofl/notosans/descripción.html",
        ]);
        let results = run_check_with_config(
            ascii_filenames,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("non-ascii-filename".to_string()),
        );
    }
}

use fontspector_checkapi::prelude::*;
use std::fs::read_to_string;

#[check(
    id = "googlefonts/metadata/escaped_strings",
    rationale = "
        
        In some cases we've seen designer names and other fields with escaped strings
        in METADATA files (such as \"Juli\\303\\241n\").

        Nowadays the strings can be full unicode strings (such as \"Julián\") and do
        not need escaping.

        Escaping quotes or double-quotes is fine, though.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2932",
    title = "Ensure METADATA.pb does not use escaped strings.",
    implementation = "all"
)]
fn escaped_strings(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let mut problems = vec![];

    for line in read_to_string(mdpb.filename.clone())?.lines() {
        // Escaped quotes are fine!
        // What we're really interested in detecting are things like
        // "Juli\303\241n" instead of "Julián"
        let mut line_string = line.to_string();
        line_string = line_string.replace("\\'", "").replace("\\\"", "");
        for quote_char in ["'", "\""] {
            let segments = line_string.split(quote_char).collect::<Vec<&str>>();
            if segments.len() >= 3 {
                #[allow(clippy::indexing_slicing)] // we just checked it's in bounds
                let a_string = segments[1];
                if a_string.contains("\\") {
                    problems.push(Status::fail(
                        "escaped-strings",
                        format!(
                            "Found escaped chars at '{a_string}'. Please use an unicode string instead."
                        )
                        .as_str(),
                    ));
                }
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use std::collections::HashMap;

    use super::escaped_strings;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        fontspector_checkapi::codetesting::run_check_with_config(
            escaped_strings,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    #[test]
    fn test_check_metadata_escaped_strings() {
        // Good: no escaped strings in METADATA.pb
        assert_pass(&run(vec![
            test_able("issue_2932/good/SomeFont-Regular.ttf"),
            test_able("issue_2932/good/METADATA.pb"),
        ]));

        // Bad: METADATA.pb contains escaped octal strings
        assert_results_contain(
            &run(vec![
                test_able("issue_2932/bad/SomeFont-Regular.ttf"),
                test_able("issue_2932/bad/METADATA.pb"),
            ]),
            StatusCode::Fail,
            Some("escaped-strings".to_string()),
        );
    }
}

use crate::checks::googlefonts::metadata::family_proto;
use fontspector_checkapi::prelude::*;

#[check(
    id = "googlefonts/metadata/category",
    rationale = "
        
        There are only five acceptable values for the category field in a METADATA.pb
        file:

        - MONOSPACE

        - SANS_SERIF

        - SERIF

        - DISPLAY

        - HANDWRITING

        This check is meant to avoid typos in this field.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2972",
    title = "Ensure METADATA.pb category field is valid.",
    implementation = "all"
)]
fn category(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let family_metadata = family_proto(mdpb)?;
    let mut problems = vec![];
    for category_value in family_metadata.category {
        if ["MONOSPACE", "SANS_SERIF", "SERIF", "DISPLAY", "HANDWRITING"]
            .iter()
            .all(|s| category_value != *s)
        {
            problems.push(Status::fail(
                "bad-value",
                &format!("The field category has \"{category_value}\" which is not valid."),
            ));
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::category;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn with_category(category_value: &str) -> Testable {
        let mdpb = test_able("cabin/METADATA.pb");
        let metadata = String::from_utf8(mdpb.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in METADATA fixture: {e}"));
        let updated = metadata.replacen(
            "category: \"SANS_SERIF\"",
            &format!("category: \"{category_value}\""),
            1,
        );
        Testable::new_with_contents("METADATA.pb", updated.into_bytes())
    }

    fn run(mdpb: Testable) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(vec![mdpb], None);
        fontspector_checkapi::codetesting::run_check_with_config(
            category,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    #[test]
    fn test_check_metadata_category() {
        assert_pass(&run(with_category("SANS_SERIF")));

        for bad in ["SAN_SERIF", "MONO_SPACE", "sans_serif", "monospace"] {
            assert_results_contain(
                &run(with_category(bad)),
                StatusCode::Fail,
                Some("bad-value".to_string()),
            );
        }

        for good in ["MONOSPACE", "SANS_SERIF", "SERIF", "DISPLAY", "HANDWRITING"] {
            assert_pass(&run(with_category(good)));
        }
    }
}

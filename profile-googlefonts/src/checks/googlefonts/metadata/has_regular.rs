use crate::checks::googlefonts::metadata::family_proto;
use fontspector_checkapi::prelude::*;

#[check(
    id = "googlefonts/metadata/has_regular",
    rationale = "
        
        According to Google Fonts standards, families should have a Regular
        style.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Ensure there is a regular style defined in METADATA.pb.",
    implementation = "all"
)]
fn has_regular(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let msg = family_proto(mdpb)?;
    if msg
        .fonts
        .iter()
        .any(|f| f.weight() == 400 && f.style() == "normal")
    {
        Ok(Status::just_one_pass())
    } else {
        Ok(Status::just_one_fail(
        "lacks-regular",
        "This family lacks a Regular (style: normal and weight: 400) as required by Google Fonts standards. If family consists of a single-weight non-Regular style only, consider the Google Fonts specs for this case: https://github.com/googlefonts/gf-docs/tree/main/Spec#single-weight-families"
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::has_regular;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check_with_config, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(mdpb: Testable) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(vec![mdpb], None);
        run_check_with_config(
            has_regular,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    #[test]
    fn test_check_metadata_has_regular() {
        assert_pass(&run(test_able("familysans/METADATA.pb")));

        let mdpb = test_able("familysans/METADATA.pb");
        let text = String::from_utf8(mdpb.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in familysans METADATA fixture: {e}"));
        let broken = text.replacen(
            "style: \"normal\"\n  weight: 400\n  filename: \"FamilySans-Regular.ttf\"",
            "style: \"normal\"\n  weight: 500\n  filename: \"FamilySans-Regular.ttf\"",
            1,
        );
        let bad = Testable::new_with_contents("METADATA.pb", broken.into_bytes());
        assert_results_contain(
            &run(bad),
            StatusCode::Fail,
            Some("lacks-regular".to_string()),
        );
    }
}

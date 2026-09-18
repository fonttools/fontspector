use crate::checks::googlefonts::metadata::family_proto;
use fontspector_checkapi::prelude::*;

#[check(
    id = "googlefonts/metadata/regular_is_400",
    rationale = "
        
        The weight of the regular style should be set to 400.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "METADATA.pb: Regular should be 400.",
    implementation = "all"
)]
fn regular_is_400(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let msg = family_proto(mdpb)?;
    let badfonts = msg
        .fonts
        .iter()
        .filter(|f| {
            f.style() == "normal"
                && (f.full_name().ends_with(" Regular")
                    || f.post_script_name().ends_with("-Regular")
                    || f.filename().contains("Regular"))
                && f.weight() != 400
        })
        .map(|f| f.filename().to_string())
        .collect::<Vec<_>>();

    if badfonts.is_empty() {
        Ok(Status::just_one_pass())
    } else {
        Ok(Status::just_one_fail(
            "not-400",
            &format!(
                "METADATA.pb: Regular font weight must be 400. Please fix these:\n\n{}",
                bullet_list(context, badfonts),
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::regular_is_400;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check_with_config, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(mdpb: Testable) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(vec![mdpb], None);
        run_check_with_config(
            regular_is_400,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    #[test]
    fn test_check_metadata_regular_is_400() {
        assert_pass(&run(test_able("familysans/METADATA.pb")));

        let mdpb = test_able("familysans/METADATA.pb");
        let text = String::from_utf8(mdpb.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in familysans METADATA fixture: {e}"));
        let broken_regular = text
            .replacen(
                "style: \"normal\"\n  weight: 400\n  filename: \"FamilySans-Regular.ttf\"\n  post_script_name: \"FamilySans-Regular\"",
                "style: \"normal\"\n  weight: 500\n  filename: \"FamilySans-Regular.ttf\"\n  post_script_name: \"FamilySans-Regular\"",
                1,
            )
            .replacen(
                "style: \"normal\"\n  weight: 500\n  filename: \"FamilySans-Medium.ttf\"\n  post_script_name: \"FamilySans-Medium\"",
                "style: \"normal\"\n  weight: 400\n  filename: \"FamilySans-Medium.ttf\"\n  post_script_name: \"FamilySans-Medium\"",
                1,
            );

        assert!(
            broken_regular.contains("weight: 500\n  filename: \"FamilySans-Regular.ttf\""),
            "Regular entry was not changed to weight 500"
        );
        assert!(
            broken_regular.contains("weight: 400\n  filename: \"FamilySans-Medium.ttf\""),
            "Medium entry was not changed to weight 400"
        );

        let bad = Testable::new_with_contents("METADATA.pb", broken_regular.into_bytes());
        assert_results_contain(&run(bad), StatusCode::Fail, Some("not-400".to_string()));
    }
}

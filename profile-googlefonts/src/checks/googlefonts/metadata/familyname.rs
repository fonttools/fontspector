use crate::checks::googlefonts::metadata::family_proto;
use fontspector_checkapi::{prelude::*, StatusCode};

#[check(
    id = "googlefonts/metadata/familyname",
    rationale = "
        
        The METADATA.pb file includes a family name field for each font
        file in the family. The value of this field should be the same
        for all fonts in the family.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Check that METADATA.pb family values are all the same.",
    implementation = "all"
)]
fn familyname(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let msg = family_proto(mdpb)?;
    assert_all_the_same(
        context,
        &(msg.fonts.iter().map(|f|
            (f.name(),
            f.name(),
            f.filename())
        ).collect::<Vec<_>>()),
        "inconsistency",
        "METADATA.pb: family name value is inconsistent across the family.\nThe following name values were found:",
        StatusCode::Fail,
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::familyname;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(mdpb: Testable) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(vec![mdpb], None);
        fontspector_checkapi::codetesting::run_check_with_config(
            familyname,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    #[test]
    fn test_check_metadata_familyname() {
        let good = test_able("familysans/METADATA.pb");
        assert_pass(&run(good));

        let original = test_able("familysans/METADATA.pb");
        let metadata = String::from_utf8(original.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in METADATA fixture: {e}"));
        let broken = metadata.replacen(
            "fonts {\n  name: \"Family Sans\"",
            "fonts {\n  name: \"Family Sans arbitrary suffix!\"",
            1,
        );
        assert_results_contain(
            &run(Testable::new_with_contents(
                "METADATA.pb",
                broken.into_bytes(),
            )),
            StatusCode::Fail,
            Some("inconsistency".to_string()),
        );
    }
}

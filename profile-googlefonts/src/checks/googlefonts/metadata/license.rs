use crate::checks::googlefonts::metadata::family_proto;
use fontspector_checkapi::prelude::*;

#[check(
    id="googlefonts/metadata/license",
    rationale="
        The license field in METADATA.pb must contain one of the
        three values \"APACHE2\", \"UFL\" or \"OFL\". (New fonts should
        generally be OFL unless there are special circumstances.)

        Additionally, the value must agree with the actual license file
        shipped with the family: an OFL.txt file means the license must
        be \"OFL\", a UFL.txt file means \"UFL\", and a LICENSE.txt file
        means \"APACHE2\". A mismatch between the METADATA.pb license
        field and the license file would publish the family under a
        license different from the one the font actually carries.
    ",
    proposal="https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    proposal="https://github.com/fonttools/fontspector/issues/778",
    title="METADATA.pb license is \"APACHE2\", \"UFL\" or \"OFL\" and matches the family's license file?",
    implementation="all"
)]
fn license(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let msg = family_proto(mdpb)?;
    let declared = msg.license();
    let mut problems = vec![];

    if declared != "APACHE2" && declared != "UFL" && declared != "OFL" {
        problems.push(Status::fail(
            "bad-license",
            &format!(
                "METADATA.pb license field (\"{declared}\") must be \"APACHE2\", \"UFL\" or \"OFL\"."
            ),
        ));
    }

    let expected_from_file = if c.get_file("OFL.txt").is_some() {
        Some(("OFL.txt", "OFL"))
    } else if c.get_file("UFL.txt").is_some() {
        Some(("UFL.txt", "UFL"))
    } else if c.get_file("LICENSE.txt").is_some() {
        Some(("LICENSE.txt", "APACHE2"))
    } else {
        None
    };

    if let Some((filename, expected)) = expected_from_file {
        if declared != expected {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "METADATA.pb license field is \"{declared}\" but the family ships a {filename} file, which means the license should be \"{expected}\"."
                ),
            ));
        }
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::license;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check_with_config, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        run_check_with_config(
            license,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    fn metadata_with_license(value: &str) -> Testable {
        let mdpb = test_able("familysans/METADATA.pb");
        let metadata = String::from_utf8(mdpb.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in METADATA.pb test fixture: {e}"));
        let updated = metadata.replacen("license: \"OFL\"", &format!("license: \"{value}\""), 1);
        Testable::new_with_contents("METADATA.pb", updated.into_bytes())
    }

    #[test]
    fn test_metadata_license_values() {
        // Standalone METADATA.pb with no license file: only the value is validated.
        assert_pass(&run(vec![metadata_with_license("APACHE2")]));
        assert_pass(&run(vec![metadata_with_license("UFL")]));
        assert_pass(&run(vec![metadata_with_license("OFL")]));

        for bad in ["APACHE", "Apache", "Ufl", "Ofl", "Open Font License"] {
            assert_results_contain(
                &run(vec![metadata_with_license(bad)]),
                StatusCode::Fail,
                Some("bad-license".to_string()),
            );
        }
    }

    #[test]
    fn test_metadata_license_matches_license_file() {
        // METADATA.pb says OFL and an OFL.txt is shipped: pass.
        assert_pass(&run(vec![
            test_able("cabinvf/METADATA.pb"),
            test_able("cabinvf/OFL.txt"),
        ]));

        // METADATA.pb says APACHE2 but an OFL.txt is shipped: mismatch.
        assert_results_contain(
            &run(vec![
                metadata_with_license("APACHE2"),
                test_able("mada/OFL.txt"),
            ]),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );

        // METADATA.pb says OFL but a LICENSE.txt (Apache) is shipped: mismatch.
        assert_results_contain(
            &run(vec![
                metadata_with_license("OFL"),
                test_able("source-sans-pro/LICENSE.txt"),
            ]),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );

        // METADATA.pb says APACHE2 and a LICENSE.txt is shipped: pass.
        assert_pass(&run(vec![
            metadata_with_license("APACHE2"),
            test_able("source-sans-pro/LICENSE.txt"),
        ]));
    }
}

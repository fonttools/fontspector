use crate::checks::googlefonts::metadata::family_proto;
use fontspector_checkapi::prelude::*;

#[check(
    id="googlefonts/metadata/license",
    rationale="
        The license field in METADATA.pb must contain one of the
        three values \"APACHE2\", \"UFL\" or \"OFL\". (New fonts should
        generally be OFL unless there are special circumstances.)
    ",
    applies_to = "MDPB",
    proposal="https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    title="METADATA.pb license is \"APACHE2\", \"UFL\" or \"OFL\"?"
)]
fn license(c: &Testable, _context: &Context) -> CheckFnResult {
    let msg = family_proto(c)?;
    if msg.license() != "APACHE2" && msg.license() != "UFL" && msg.license() != "OFL" {
        Ok(Status::just_one_fail(
            "bad-license",
            &format!(
                "'METADATA.pb license field (\"{}\") must be \"APACHE2\", \"UFL\" or \"OFL\".",
                msg.license()
            ),
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
mod tests {
    use super::license;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode, Testable,
    };

    fn with_license(value: &str) -> Testable {
        let mdpb = test_able("familysans/METADATA.pb");
        let metadata = String::from_utf8(mdpb.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in METADATA.pb test fixture: {e}"));
        let updated = metadata.replacen("license: \"OFL\"", &format!("license: \"{value}\""), 1);
        Testable::new_with_contents("METADATA.pb", updated.into_bytes())
    }

    #[test]
    fn test_metadata_license_values() {
        assert_pass(&run_check(license, with_license("APACHE2")));
        assert_pass(&run_check(license, with_license("UFL")));
        assert_pass(&run_check(license, with_license("OFL")));

        for bad in ["APACHE", "Apache", "Ufl", "Ofl", "Open Font License"] {
            let results = run_check(license, with_license(bad));
            assert_results_contain(&results, StatusCode::Fail, Some("bad-license".to_string()));
        }
    }
}

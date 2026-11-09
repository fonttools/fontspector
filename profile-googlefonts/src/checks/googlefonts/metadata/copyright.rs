use crate::checks::googlefonts::metadata::family_proto;
use fontspector_checkapi::{prelude::*, StatusCode};

#[check(
    id="googlefonts/metadata/copyright",
    rationale="
        The METADATA.pb file includes a copyright field for each font
        file in the family. The value of this field should be the same
        for all fonts in the family.
    ",
    applies_to = "MDPB",
    proposal="https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    title="METADATA.pb: Copyright notice is the same in all fonts?"
)]
fn copyright(c: &Testable, context: &Context) -> CheckFnResult {
    let msg = family_proto(c)?;
    assert_all_the_same(
        context,
        &(msg.fonts.iter().map(|f|
            (f.copyright(),
            f.copyright(),
            f.filename())
        ).collect::<Vec<_>>()),
        "inconsistency",
        "METADATA.pb: Copyright field value is inconsistent across the family.\nThe following copyright values were found:",
        StatusCode::Fail,
    )
}

#[cfg(test)]
mod tests {
    use super::copyright;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_check_metadata_copyright() {
        assert_pass(&run_check(copyright, test_able("familysans/METADATA.pb")));

        assert_results_contain(
            &run_check(copyright, test_able("familysans/bad-METADATA.pb")),
            StatusCode::Fail,
            Some("inconsistency".to_string()),
        );
    }
}

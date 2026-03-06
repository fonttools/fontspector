use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "dsig",
    title = "Does the font have a DSIG table?",
    rationale = "
        The DSIG (Digital Signature) table is only required for old programs
        like MS Office 2013 in order to work properly. The current
        recommendation is to completely remove the DSIG table, as it provides
        no real security benefit and some foundries deliberately omit it.

        This check is separate from the general unwanted_tables check so that
        it can be independently disabled by foundries that choose to retain
        the DSIG table.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/101"
)]
fn dsig(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    if f.has_table(b"DSIG") {
        Ok(Status::just_one_warn(
            "found-DSIG",
            "This font has a digital signature (DSIG table) which \
             is only required — even if only a placeholder — on old \
             programs like MS Office 2013 in order to work properly. \
             The current recommendation is to completely remove the \
             DSIG table.",
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontations::write::{tables::maxp::Maxp, FontBuilder};
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check},
        StatusCode, Testable,
    };

    use super::dsig;

    #[test]
    fn test_no_dsig_passes() {
        // Build a minimal font without DSIG
        let mut builder = FontBuilder::new();
        builder.add_table(&Maxp::default()).unwrap();
        let testable = Testable::new_with_contents("demo.ttf", builder.build().clone());
        let results = run_check(dsig, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_with_dsig_warns() {
        // Build a minimal font and add a dummy DSIG table
        let mut builder = FontBuilder::new();
        builder.add_table(&Maxp::default()).unwrap();
        // Add a minimal DSIG table (version 1, no signatures)
        let dsig_data: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x01, // version
            0x00, 0x00, // numSignatures
            0x00, 0x00, // flags
        ];
        builder.add_raw(
            fontations::skrifa::raw::types::Tag::new(b"DSIG"),
            &dsig_data,
        );
        let testable = Testable::new_with_contents("demo.ttf", builder.build().clone());
        let results = run_check(dsig, testable);
        assert_results_contain(&results, StatusCode::Warn, Some("found-DSIG".to_string()));
    }
}

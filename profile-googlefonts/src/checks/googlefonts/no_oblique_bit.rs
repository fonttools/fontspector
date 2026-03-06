use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "googlefonts/no_oblique_bit",
    title = "Ensure the OS/2 OBLIQUE fsSelection bit is not set.",
    rationale = "
        Google Fonts does not want fonts with the OBLIQUE bit (bit 9) set
        in the OS/2 fsSelection field. Fonts that are oblique should use
        the Italic bit instead, or be served as a separate Italic file.

        Some fonts like Red Hat Text have been found with both the Italic
        and Oblique bits set, which is not desired for the Google Fonts
        collection.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/238"
)]
fn no_oblique_bit(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let flags = f.get_os2_fsselection()?;
    if flags.contains(fontations::skrifa::raw::tables::os2::SelectionFlags::OBLIQUE) {
        let message = "The OS/2 fsSelection OBLIQUE bit (bit 9) is set. \
             Google Fonts does not want this bit enabled. \
             Oblique styles should use the Italic bit instead.";
        let mut status = Status::fail("oblique-bit-set", message);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("fsSelection".to_string()),
            actual: Some(json!("OBLIQUE bit set")),
            expected: Some(json!("OBLIQUE bit not set")),
            message: message.to_string(),
        });
        return_result(vec![status])
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    use super::no_oblique_bit;

    #[test]
    fn test_pass_no_oblique() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(no_oblique_bit, testable);
        assert_pass(&results);
    }
}

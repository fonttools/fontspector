use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "no_mac_entries",
    rationale = "
        Mac name table entries are not needed anymore. Even Apple stopped producing
        name tables with platform 1. Please see for example the following system font:

        /System/Library/Fonts/SFCompact.ttf

        Also, Dave Opstad, who developed Apple's TrueType specifications, told
        Olli Meier a couple years ago (as of January/2022) that these entries are
        outdated and should not be produced anymore.",
    proposal = "https://github.com/googlefonts/gftools/issues/469",
    title = "Ensure font doesn't have Mac name table entries (platform=1)."
)]
fn no_mac_entries(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    for rec in f.font().name()?.name_record() {
        if rec.platform_id() == 1 {
            problems.push(Status::fail(
                "mac-names",
                &format!("Please remove name ID {}", rec.name_id()),
            ))
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_no_mac_entries() {
        // Test with a font that has no Mac names.
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Regular.otf");
        let results = run_check(no_mac_entries, testable);
        assert_pass(&results);

        // Test with a font that has Mac names.
        let testable_with_mac = test_able("abeezee/ABeeZee-Italic.ttf");
        let results_with_mac = run_check(no_mac_entries, testable_with_mac);
        assert_results_contain(
            &results_with_mac,
            StatusCode::Fail,
            Some("mac-names".to_string()),
        );
    }
}

use fontations::skrifa::raw::{tables::os2::SelectionFlags, TableProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

/// fsSelection flags that are only valid for OS/2 version >= 4.
const V4_FLAGS: &[(SelectionFlags, &str)] = &[
    (SelectionFlags::USE_TYPO_METRICS, "USE_TYPO_METRICS (bit 7)"),
    (SelectionFlags::WWS, "WWS (bit 8)"),
    (SelectionFlags::OBLIQUE, "OBLIQUE (bit 9)"),
];

#[check(
    id = "opentype/os2_version_fsselection",
    title = "OS/2 version must be >= 4 if certain fsSelection bits are set.",
    rationale = "
        The fsSelection bits USE_TYPO_METRICS (bit 7), WWS (bit 8), and
        OBLIQUE (bit 9) were introduced in OS/2 version 4. Setting these
        bits in an earlier version of the OS/2 table is invalid and may
        cause unpredictable behavior in some environments.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/178"
)]
fn os2_version_fsselection(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let os2 = f.font().os2()?;
    let version = os2.version();
    let flags = os2.fs_selection();
    let mut problems = vec![];

    if version < 4 {
        for (flag, name) in V4_FLAGS {
            if flags.contains(*flag) {
                let message = format!(
                    "OS/2 version is {}, but fsSelection {} is set. \
                     This bit requires OS/2 version >= 4.",
                    version, name
                );
                let mut status = Status::warn("v4-flag-in-old-os2", &message);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "OS/2".to_string(),
                    field_name: Some("version / fsSelection".to_string()),
                    actual: Some(json!({"version": version, "flag": name})),
                    expected: Some(json!("version >= 4 for this flag")),
                    message: message.clone(),
                });
                problems.push(status);
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    use super::os2_version_fsselection;

    #[test]
    fn test_pass_normal_font() {
        // Mada-Regular should pass (typical OS/2 version >= 4)
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(os2_version_fsselection, testable);
        assert_pass(&results);
    }
}

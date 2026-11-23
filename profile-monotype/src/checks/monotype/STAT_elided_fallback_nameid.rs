use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "monotype/STAT_elided_fallback_nameid",
    rationale = "
        STAT elidedFallbackNameID is the name applications use when no axis
        labels apply. It must resolve to a real, non-empty name table entry,
        and Monotype requires that name to equal the Subfamily Name (name ID 2),
        matching FontSniffer clearance behaviour.
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/35",
    title = "Does STAT elidedFallbackNameID resolve to a real name and equal name ID 2?"
)]
fn STAT_elided_fallback_nameid(t: &Testable, _context: &Context) -> CheckFnResult {
    let _f = testfont!(t);
    // Intentionally incomplete for TDD red step
    Ok(Status::just_one_fatal(
        "elided-name-not-in-name-table",
        "stub",
    ))
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::STAT_elided_fallback_nameid;
    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    #[test]
    fn test_STAT_elided_fallback_nameid_pass() {
        let testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let results = run_check(STAT_elided_fallback_nameid, testable);
        assert_pass(&results);
    }
}

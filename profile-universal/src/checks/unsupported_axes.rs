use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "varfont/unsupported_axes",
    rationale = "
        The 'ital' axis is not supported yet in Google Chrome.

        For the time being, we need to ensure that VFs do not contain this axis.
        Once browser support is better, we can deprecate this check.

        For more info regarding browser support, see:
        https://arrowtype.github.io/vf-slnt-test/
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2866",
    title = "Ensure VFs do not contain (yet) the ital axis."
)]
fn unsupported_axes(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font"
    );
    Ok(if f.font().axes().iter().any(|a| a.tag() == "ital") {
        Status::just_one_fail(
            "unsupported-ital",
            "The 'ital' axis is not supported yet in Google Chrome.",
        )
    } else {
        Status::just_one_pass()
    })
}

#[cfg(test)]
mod tests {
    use super::unsupported_axes;
    use fontspector_checkapi::codetesting::{assert_pass, assert_skip, run_check, test_able};

    #[test]
    fn test_unsupported_axes_pass() {
        let testable = test_able("ibmplexsans-vf/IBMPlexSansVar-Roman.ttf");
        let results = run_check(unsupported_axes, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_unsupported_axes_skip_not_variable() {
        let testable = test_able("cabin/Cabin-Regular.ttf");
        let results = run_check(unsupported_axes, testable);
        assert_skip(&results);
    }
}

use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "mandatory_avar_table",
    rationale = "
        Most variable fonts should include an avar table to correctly define
        axes progression rates.

        For example, a weight axis from 0% to 100% doesn't map directly to 100 to 1000,
        because a 10% progression from 0% may be too much to define the 200,
        while 90% may be too little to define the 900.

        If the progression rates of axes is linear, this check can be ignored.
        Fontmake will also skip adding an avar table if the progression rates
        are linear. However, it is still recommended that designers visually proof
        each instance is at the expected weight, width etc.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3100",
    title = "Ensure variable fonts include an avar table."
)]
fn mandatory_avar_table(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font"
    );
    Ok(if f.has_table(b"avar") {
        Status::just_one_pass()
    } else {
        Status::just_one_warn("missing-avar", "The font does not include an avar table.  If the progression rates of axes is linear and no user-mapping is expected, this is fine, and this check can be ignored or excluded.")
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::mandatory_avar_table;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, remove_table, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_mandatory_avar_pass() {
        let testable = test_able("ibmplexsans-vf/IBMPlexSansVar-Roman.ttf");
        let results = run_check(mandatory_avar_table, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_mandatory_avar_warn_missing() {
        let mut testable = test_able("ibmplexsans-vf/IBMPlexSansVar-Roman.ttf");
        remove_table(&mut testable, b"avar");
        let results = run_check(mandatory_avar_table, testable);
        assert_results_contain(&results, StatusCode::Warn, Some("missing-avar".to_string()));
    }
}

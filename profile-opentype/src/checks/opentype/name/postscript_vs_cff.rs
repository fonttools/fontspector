use fontations::skrifa::raw::{types::NameId, TableProvider};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "opentype/name/postscript_vs_cff",
    rationale = "
        The PostScript name entries in the font's 'name' table should match
        the FontName string in the 'CFF ' table.

        The 'CFF ' table has a lot of information that is duplicated in other tables.
        This information should be consistent across tables, because there's
        no guarantee which table an app will get the data from.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/2229",
    title = "CFF table FontName must match name table ID 6 (PostScript name)."
)]
fn postscript_vs_cff(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    skip!(
        !font.has_table(b"CFF "),
        "no-cff",
        "This check only applies to CFF fonts."
    );
    if font.font().cff()?.names().count() > 1 {
        return Ok(Status::just_one_fail(
            "cff-name-error",
            "Unexpected number of font names in CFF table.",
        ));
    }
    let cff_name = String::from_utf8_lossy(
        font.font()
            .cff()?
            .names()
            .get(0)
            .map_err(|e| FontspectorError::General(format!("Error reading CFF table: {e}")))?,
    );
    let name = font.get_name_entry_strings(NameId::POSTSCRIPT_NAME).next();
    if let Some(name) = name {
        if cff_name != name {
            return Ok(Status::just_one_fail(
                "ps-cff-name-mismatch",
                &format!(
                    "Name table PostScript name '{name}' does not match CFF table FontName '{cff_name}'.",
                ),
            ));
        }
    }
    Ok(Status::just_one_pass())
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontations::skrifa::raw::types::NameId;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    #[test]
    fn test_postscript_vs_cff_pass() {
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Bold.otf");
        let result = run_check(postscript_vs_cff, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_postscript_vs_cff_fail_mismatch() {
        let mut testable = test_able("source-sans-pro/OTF/SourceSansPro-Bold.otf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::POSTSCRIPT_NAME,
            "SomeOtherFontName".to_string(),
        );
        let result = run_check(postscript_vs_cff, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("ps-cff-name-mismatch".to_string()),
        );
    }

    #[test]
    fn test_postscript_vs_cff_skip_ttf() {
        let testable = test_able("source-sans-pro/TTF/SourceSansPro-Bold.ttf");
        let result = run_check(postscript_vs_cff, testable);
        assert_results_contain(&result, StatusCode::Skip, Some("no-cff".to_string()));
    }

    #[test]
    fn test_postscript_vs_cff_skip_cff2() {
        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Italic.otf");
        let result = run_check(postscript_vs_cff, testable);
        assert_results_contain(&result, StatusCode::Skip, Some("no-cff".to_string()));
    }
}

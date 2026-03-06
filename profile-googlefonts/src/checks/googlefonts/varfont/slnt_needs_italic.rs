use fontations::skrifa::{raw::types::Tag, MetadataProvider};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "googlefonts/varfont/slnt_needs_italic",
    rationale = "
        Google Workspace applications (Docs, Sheets, Slides) do not currently
        support the 'slnt' (Slant) axis for selecting italic styles. If a
        variable font family relies solely on a Slant axis for its italic
        styles, those styles will not be accessible in Workspace apps.

        To ensure full compatibility, font families with a Slant axis should
        also provide a separate Italic file so that Workspace users can access
        italic styles through the standard Bold/Italic toggles.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/647",
    title = "Variable fonts with slnt axis need a separate Italic file for Google Workspace compatibility."
)]
fn slnt_needs_italic(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font."
    );
    let has_slnt = f
        .font()
        .axes()
        .iter()
        .any(|a| a.tag() == Tag::new(b"slnt"));
    skip!(!has_slnt, "no-slnt", "Font does not have a 'slnt' axis.");
    let has_ital = f
        .font()
        .axes()
        .iter()
        .any(|a| a.tag() == Tag::new(b"ital"));
    if has_ital {
        Ok(Status::just_one_pass())
    } else {
        Ok(Status::just_one_warn(
            "needs-italic",
            "This variable font has a 'slnt' (Slant) axis but no 'ital' axis. \
             Google Workspace apps (Docs, Sheets, Slides) do not support the \
             Slant axis for selecting italic styles. Please ensure this font \
             family also includes a separate Italic file so that italic styles \
             are accessible in Workspace applications.",
        ))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontations::{
        skrifa::raw::TableProvider,
        write::{
            from_obj::ToOwnedTable,
            tables::fvar::{Fvar, VariationAxisRecord},
            FontBuilder,
        },
    };
    use fontspector_checkapi::{
        codetesting::{assert_results_contain, assert_skip, run_check, test_able},
        FileTypeConvert, StatusCode, TTF,
    };

    use super::slnt_needs_italic;

    #[test]
    fn test_slnt_without_ital_warns() {
        // Inter has slnt and wght axes but no ital axis
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(slnt_needs_italic, testable);
        assert_results_contain(&results, StatusCode::Warn, Some("needs-italic".to_string()));
    }

    #[test]
    fn test_skip_non_variable() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(slnt_needs_italic, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_skip_no_slnt_axis() {
        // A variable font without slnt axis should be skipped
        let testable = test_able("leaguegothic-vf/LeagueGothic[wdth].ttf");
        let results = run_check(slnt_needs_italic, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_slnt_with_ital_passes() {
        // Start with Inter (has slnt+wght), add an ital axis and clear instances
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();

        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();
        fvar.axis_instance_arrays.axes.push(VariationAxisRecord {
            axis_tag: fontations::write::types::Tag::new(b"ital"),
            min_value: fontations::write::types::Fixed::from_f64(0.0),
            default_value: fontations::write::types::Fixed::from_f64(0.0),
            max_value: fontations::write::types::Fixed::from_f64(1.0),
            flags: 0,
            axis_name_id: fontations::write::types::NameId::new(256),
        });
        // Clear instances to avoid axis_count mismatch
        fvar.axis_instance_arrays.instances.clear();

        let new_bytes = FontBuilder::new()
            .add_table(&fvar)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();

        testable.contents = new_bytes;

        let results = run_check(slnt_needs_italic, testable);
        assert_results_contain(&results, StatusCode::Pass, None);
    }
}

use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::MetadataProvider;

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
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::unsupported_axes;
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, assert_skip, run_check, test_able,
    };
    use fontspector_checkapi::{FileTypeConvert, StatusCode, TTF};
    use skrifa::raw::TableProvider;
    use write_fonts::from_obj::ToOwnedTable;
    use write_fonts::tables::fvar::{Fvar, VariationAxisRecord};
    use write_fonts::types::{Fixed, NameId, Tag};

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

    #[test]
    fn test_unsupported_axes_fail_ital() {
        let mut testable = test_able("cabinvfbeta/CabinVFBeta.ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();
        fvar.axis_instance_arrays
            .axes
            .push(VariationAxisRecord::new(
                Tag::new(b"ital"),
                Fixed::from_f64(0.0),
                Fixed::from_f64(0.0),
                Fixed::from_f64(1.0),
                0,
                NameId::new(256),
            ));
        // Every instance needs a coordinate for the new axis
        for instance in &mut fvar.axis_instance_arrays.instances {
            instance.coordinates.push(Fixed::from_f64(0.0));
        }
        let new_bytes = f.rebuild_with_new_table(&fvar).unwrap();
        testable.set(new_bytes);

        let results = run_check(unsupported_axes, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("unsupported-ital".to_string()),
        );
    }
}

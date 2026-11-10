use std::collections::HashSet;

use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::{raw::TableProvider, MetadataProvider};

#[check(
    id = "opentype/varfont/STAT_axis_record_for_each_axis",
    rationale = "
        According to the OpenType spec, there must be an Axis Record
        for every axis defined in the fvar table.

        https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-records
    ",
    title = "All fvar axes have a correspondent Axis Record on STAT table?",
    proposal = "https://github.com/fonttools/fontbakery/pull/3017"
)]
fn STAT_axis_record_for_each_axis(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(!f.is_variable_font(), "not-variable", "Not a variable font");
    let fvar_axis_tags: HashSet<_> = f
        .font()
        .axes()
        .iter()
        .map(|axis| axis.tag().to_string())
        .collect();
    let stat_axis_tags: HashSet<_> = f
        .font()
        .stat()
        .map_err(|_| FontspectorError::skip("no-stat", "No STAT table"))?
        .design_axes()?
        .iter()
        .map(|axis_record| axis_record.axis_tag().to_string())
        .collect();
    let missing_axes: Vec<&str> = fvar_axis_tags
        .difference(&stat_axis_tags)
        .map(|x| x.as_ref())
        .collect();
    Ok(if missing_axes.is_empty() {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "missing-axis-records",
            &format!(
                "STAT table is missing Axis Records for the following axes:\n\n{}",
                bullet_list(context, &missing_axes)
            ),
        )
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{
        assert_messages_contain, assert_pass, assert_results_contain, assert_skip, run_check,
        test_able,
    };
    use fontspector_checkapi::{FileTypeConvert, StatusCode, TTF};
    use skrifa::raw::TableProvider;
    use write_fonts::from_obj::ToOwnedTable;
    use write_fonts::tables::stat::Stat;
    use write_fonts::types::Tag;

    #[test]
    fn test_stat_axis_record_pass() {
        let testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let result = run_check(super::STAT_axis_record_for_each_axis, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_stat_axis_record_not_variable_skip() {
        let testable = test_able("source-sans-pro/TTF/SourceSansPro-Black.ttf");
        let result = run_check(super::STAT_axis_record_for_each_axis, testable);
        assert_skip(&result);
    }

    #[test]
    fn test_stat_axis_record_missing_axis() {
        let mut testable = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut stat: Stat = f.font().stat().unwrap().to_owned_table();
        // Remove the first Axis Record (`wdth`)
        stat.design_axes
            .retain(|axis| axis.axis_tag != Tag::new(b"wdth"));
        let new_bytes = f.rebuild_with_new_table(&stat).unwrap();
        testable.set(new_bytes);

        let result = run_check(super::STAT_axis_record_for_each_axis, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("missing-axis-records".to_string()),
        );
        assert_messages_contain(&result, "* wdth");
    }
}

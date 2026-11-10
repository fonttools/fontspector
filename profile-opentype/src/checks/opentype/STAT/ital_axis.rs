use fontspector_checkapi::{prelude::*, FileTypeConvert, TestFont};
use skrifa::raw::{
    tables::stat::{AxisValue, AxisValueTableFlags},
    ReadError, TableProvider,
};
use skrifa::MetadataProvider;
use write_fonts::types::NameId;

fn segment_collection(fonts: Vec<TestFont>) -> Vec<(Option<TestFont>, Option<TestFont>)> {
    let mut roman_italic = vec![];
    let (italics, mut non_italics): (Vec<_>, Vec<_>) = fonts
        .into_iter()
        .partition(|f| f.is_italic().unwrap_or(false));
    for italic in italics.into_iter() {
        // Find a matching roman
        if let Some(name) = italic
            .font()
            .localized_strings(NameId::FAMILY_NAME)
            .english_or_first()
        {
            let suspected_roman_family_name = name.to_string();
            if let Some(index) = non_italics.iter().position(|f| {
                if let Some(other_name) = f
                    .font()
                    .localized_strings(NameId::FAMILY_NAME)
                    .english_or_first()
                {
                    other_name.to_string() == suspected_roman_family_name
                } else {
                    false
                }
            }) {
                let roman = non_italics.swap_remove(index);
                roman_italic.push((Some(roman), Some(italic)));
            } else {
                roman_italic.push((None, Some(italic)));
            }
        }
    }
    // Now add all the remaining non-italic fonts
    for roman in non_italics.into_iter() {
        roman_italic.push((Some(roman), None));
    }

    roman_italic
}

fn check_has_ital(t: &TestFont) -> Option<Status> {
    if let Ok(stat) = t.font().stat() {
        let has_ital = stat
            .design_axes()
            .ok()?
            .iter()
            .any(|axis| axis.axis_tag() == "ital");
        if !has_ital {
            Some(Status::fail(
                "missing-ital-axis",
                &format!(
                    "Font {} lacks an 'ital' axis in the STAT table.",
                    t.filename.to_string_lossy()
                ),
            ))
        } else {
            None
        }
    } else {
        Some(Status::fail(
            "no-stat",
            &format!("Font {} has no STAT table", t.filename.to_string_lossy()),
        ))
    }
}

// This is horrible because the structure of STAT table value records is horrible.
fn check_ital_is_binary_and_last(t: &TestFont, is_italic: bool) -> Result<Vec<Status>, ReadError> {
    let mut problems = vec![];
    if let Ok(stat) = t.font().stat() {
        let axes = stat.design_axes()?;
        if let Some(ital_pos) = axes.iter().position(|axis| axis.axis_tag() == "ital") {
            if ital_pos != axes.len() - 1 {
                problems.push(Status::warn(
                    "ital-axis-not-last",
                    &format!(
                        "Font {} has 'ital' axis in position {} of {}.",
                        t.filename.to_string_lossy(),
                        ital_pos + 1,
                        axes.len()
                    ),
                ));
            }

            let expected_value = if is_italic { 1.0 } else { 0.0 };
            let expected_flags = if is_italic {
                AxisValueTableFlags::empty()
            } else {
                AxisValueTableFlags::ELIDABLE_AXIS_VALUE_NAME
            };
            if let Some(Ok(subtable)) = stat.offset_to_axis_values() {
                for val in subtable.axis_values().iter().flatten() {
                    match &val {
                        AxisValue::Format1(v) => {
                            if v.axis_index() != ital_pos as u16 {
                                continue;
                            }
                            if v.value().to_f32() != expected_value {
                                problems.push(Status::warn(
                                    "wrong-ital-axis-value",
                                    &format!(
                                        "{} has STAT table 'ital' axis with wrong value. Expected: {}, got '{}'",
                                        t.filename.to_string_lossy(),
                                        expected_value,
                                        v.value()
                                    ),
                                ))
                            }
                            if val.flags() != expected_flags {
                                problems.push(Status::warn(
                                    "wrong-ital-axis-flag",
                                    &format!(
                                        "{} has STAT table 'ital' axis with wrong flags. Expected: {:?}, got '{:?}'",
                                        t.filename.to_string_lossy(),expected_flags,val.flags()
                                    ),
                                ))
                            }
                        }
                        AxisValue::Format2(v) => {
                            if v.axis_index() != ital_pos as u16 {
                                continue;
                            }
                            if v.nominal_value().to_f32() != expected_value {
                                problems.push(Status::warn(
                                    "wrong-ital-axis-value",
                                    &format!(
                                        "{} has STAT table 'ital' axis with wrong value. Expected: {}, got '{}'",
                                        t.filename.to_string_lossy(),
                                        expected_value,
                                        v.nominal_value()
                                    ),
                                ))
                            }
                            if val.flags() != expected_flags {
                                problems.push(Status::warn(
                                    "wrong-ital-axis-flag",
                                    &format!(
                                        "{} has STAT table 'ital' axis with wrong flags. Expected: {:?}, got '{:?}'",
                                        t.filename.to_string_lossy(),expected_flags,val.flags()
                                    ),
                                ))
                            }
                        }
                        AxisValue::Format3(v) => {
                            if v.axis_index() != ital_pos as u16 {
                                continue;
                            }
                            if v.value().to_f32() != expected_value {
                                problems.push(Status::warn(
                                    "wrong-ital-axis-value",
                                    &format!(
                                        "{} has STAT table 'ital' axis with wrong value. Expected: {}, got '{}'",
                                        t.filename.to_string_lossy(),
                                        expected_value,
                                        v.value()
                                    ),
                                ))
                            }
                            if val.flags() != expected_flags {
                                problems.push(Status::warn(
                                    "wrong-ital-axis-flag",
                                    &format!(
                                        "{} has STAT table 'ital' axis with wrong flags. Expected: {:?}, got '{:?}'",
                                        t.filename.to_string_lossy(),expected_flags,val.flags()
                                    ),
                                ))
                            }
                            // If we are Roman, check for the linked value
                            if !is_italic {
                                let linked_value = v.linked_value();
                                if linked_value.to_f32() != 1.0 {
                                    problems.push(Status::warn(
                                            "wrong-ital-axis-linkedvalue",
                                            &format!(
                                                "{} has STAT table 'ital' axis with wrong linked value. Expected: 1.0, got '{}'",
                                                t.filename.to_string_lossy(),
                                                linked_value
                                            ),
                                        ))
                                }
                            }
                        }
                        AxisValue::Format4(_) => {
                            // We don't handle this.
                        }
                    }
                }
            }
        }
    }
    Ok(problems)
}

#[check(
    id = "opentype/STAT/ital_axis",
    rationale = "
        Check that related Upright and Italic have an
        'ital' axis in the STAT table.

        Since the STAT table can be used to create new instances, it is
        important to ensure that such an 'ital' axis be the last one
        declared in the STAT table so that the eventual naming of new
        instances follows the subfamily traditional scheme (RIBBI / WWS)
        where \"Italic\" is always last.

        The 'ital' axis should also be strictly boolean, only accepting
        values of 0 (for Uprights) or 1 (for Italics). This usually works
        as a mechanism for selecting between two linked variable font files.

        Also, the axis value name for uprights must be set as elidable.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2934",
    proposal = "https://github.com/fonttools/fontbakery/issues/3668",
    proposal = "https://github.com/fonttools/fontbakery/issues/3669",
    implementation = "all",
    title = "Ensure Fonts have 'ital' STAT axis."
)]
fn ital_axis(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(c);

    for font in fonts.iter() {
        if font.has_table(b"gvar") && !font.has_table(b"STAT") {
            // variable font must have a STAT table
            return Ok(Status::just_one_fail(
                "no-stat-table",
                "Variable font is missing the 'STAT' table.",
            ));
        } else if !font.has_table(b"gvar") && !font.has_table(b"STAT") {
            // static font is recommend to have a STAT table
            return Ok(Status::just_one_warn(
                "no-stat-table",
                "Static font is missing the 'STAT' table.",
            ));
        }
    }

    let mut problems = vec![];

    for pair in segment_collection(fonts).into_iter() {
        match pair {
            (Some(roman), Some(italic)) => {
                // These should definitely both have an ital axis
                problems.extend(check_has_ital(&roman));
                problems.extend(check_has_ital(&italic));
                problems.extend(check_ital_is_binary_and_last(&roman, false)?);
                problems.extend(check_ital_is_binary_and_last(&italic, true)?);
            }
            (None, Some(italic)) => {
                // Standalone italic font — validate its ital axis values
                problems.extend(check_ital_is_binary_and_last(&italic, true)?);
            }
            (None, None) => {}
            (Some(roman), None) => {
                problems.extend(check_ital_is_binary_and_last(&roman, false)?);
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_pass, assert_results_contain, run_check_with_config,
            test_able,
        },
        StatusCode, Testable, TestableType,
    };
    use std::collections::HashMap;
    use write_fonts::from_obj::ToOwnedTable;
    use write_fonts::tables::stat::{AxisValue as WriteAxisValue, Stat};
    use write_fonts::types::Tag;

    fn run_on(fonts: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection {
            testables: fonts,
            directory: "".to_string(),
        };
        run_check_with_config(
            super::ital_axis,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    /// Apply a mutation to the font's STAT table and rebuild the font.
    fn with_modified_stat(t: &mut Testable, f: impl FnOnce(&mut Stat)) {
        let font = TTF.from_testable(t).unwrap();
        let mut stat: Stat = font.font().stat().unwrap().to_owned_table();
        f(&mut stat);
        let new_bytes = font.rebuild_with_new_table(&stat).unwrap();
        t.set(new_bytes);
    }

    /// Apply a mutation to every STAT axis value belonging to the `ital` axis.
    fn for_ital_axis_values(stat: &mut Stat, mut f: impl FnMut(&mut WriteAxisValue)) {
        let Some(ital_pos) = stat
            .design_axes
            .iter()
            .position(|axis| axis.axis_tag == Tag::new(b"ital"))
        else {
            return;
        };
        if let Some(values) = stat.offset_to_axis_values.as_mut() {
            for value in values.iter_mut() {
                // Format 4 combines several axes and is ignored by the check, so
                // we do not treat it as belonging to the 'ital' axis.
                let axis_index = match &**value {
                    WriteAxisValue::Format1(v) => Some(v.axis_index),
                    WriteAxisValue::Format2(v) => Some(v.axis_index),
                    WriteAxisValue::Format3(v) => Some(v.axis_index),
                    WriteAxisValue::Format4(_) => None,
                };
                if axis_index == Some(ital_pos as u16) {
                    f(value);
                }
            }
        }
    }

    fn set_ital_axis_value(stat: &mut Stat, new_value: f32) {
        for_ital_axis_values(stat, |value| match value {
            WriteAxisValue::Format1(v) => {
                v.value = write_fonts::types::Fixed::from_f64(new_value as f64)
            }
            WriteAxisValue::Format2(v) => {
                v.nominal_value = write_fonts::types::Fixed::from_f64(new_value as f64)
            }
            WriteAxisValue::Format3(v) => {
                v.value = write_fonts::types::Fixed::from_f64(new_value as f64)
            }
            WriteAxisValue::Format4(_) => {}
        });
    }

    fn set_ital_axis_flags(stat: &mut Stat, new_flags: AxisValueTableFlags) {
        for_ital_axis_values(stat, |value| match value {
            WriteAxisValue::Format1(v) => v.flags = new_flags,
            WriteAxisValue::Format2(v) => v.flags = new_flags,
            WriteAxisValue::Format3(v) => v.flags = new_flags,
            WriteAxisValue::Format4(v) => v.flags = new_flags,
        });
    }

    fn set_ital_axis_linked_value(stat: &mut Stat, new_value: f32) {
        for_ital_axis_values(stat, |value| {
            if let WriteAxisValue::Format3(v) = value {
                v.linked_value = write_fonts::types::Fixed::from_f64(new_value as f64);
            }
        });
    }

    fn remove_ital_axis(stat: &mut Stat) {
        stat.design_axes
            .retain(|axis| axis.axis_tag != Tag::new(b"ital"));
    }

    fn move_ital_axis_to_front(stat: &mut Stat) {
        let ital = stat
            .design_axes
            .iter()
            .find(|axis| axis.axis_tag == Tag::new(b"ital"))
            .cloned();
        if let Some(ital) = ital {
            stat.design_axes
                .retain(|axis| axis.axis_tag != Tag::new(b"ital"));
            stat.design_axes.insert(0, ital);
        }
    }

    fn roman() -> Testable {
        test_able("shantell/ShantellSans[BNCE,INFM,SPAC,wght].ttf")
    }

    fn italic() -> Testable {
        test_able("shantell/ShantellSans-Italic[BNCE,INFM,SPAC,wght].ttf")
    }

    #[test]
    fn test_stat_ital_axis_pass() {
        let result = run_on(vec![roman(), italic()]);
        assert_pass(&result);
    }

    #[test]
    fn test_stat_ital_axis_standalone_italic_pass() {
        // A standalone italic with a valid ital axis should pass
        let result = run_on(vec![italic()]);
        assert_pass(&result);
    }

    #[test]
    fn test_stat_ital_axis_roman_only_pass() {
        let result = run_on(vec![roman()]);
        assert_pass(&result);
    }

    #[test]
    fn test_ital_axis_static_fonts_missing_stat() {
        let result = run_on(vec![
            test_able("cabin/Cabin-Regular.ttf"),
            test_able("cabin/Cabin-Italic.ttf"),
        ]);
        assert_results_contain(&result, StatusCode::Warn, Some("no-stat-table".to_string()));
    }

    #[test]
    fn test_ital_axis_static_fonts_with_stat() {
        let result = run_on(vec![
            test_able("notosans/static/NotoSans-Black.ttf"),
            test_able("notosans/static/NotoSans-BlackItalic.ttf"),
        ]);
        assert_results_contain(&result, StatusCode::Pass, None);
    }

    #[test]
    fn test_segment_collection_static() {
        let testables: Vec<Testable> = vec![
            test_able("montserrat/Montserrat-Regular.ttf"),
            test_able("montserrat/Montserrat-Italic.ttf"),
            test_able("montserrat/Montserrat-Bold.ttf"),
            test_able("montserrat/Montserrat-BoldItalic.ttf"),
            test_able("montserrat/Montserrat-Light.ttf"),
            test_able("montserrat/Montserrat-LightItalic.ttf"),
        ];
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let fonts = TTF.from_collection(&collection);
        let pairs = segment_collection(fonts);
        assert_eq!(pairs.len(), 3);
        for (roman, italic) in pairs.into_iter() {
            assert!(roman.is_some());
            assert!(italic.is_some());
        }
    }

    #[test]
    fn test_stat_ital_axis_missing_ital_axis() {
        // Removing the ital axis from both fonts should FAIL.
        let mut roman = roman();
        let mut italic = italic();
        with_modified_stat(&mut roman, remove_ital_axis);
        with_modified_stat(&mut italic, remove_ital_axis);

        let result = run_on(vec![roman, italic]);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("missing-ital-axis".to_string()),
        );
    }

    #[test]
    fn test_stat_ital_axis_wrong_value_roman() {
        // Roman fonts must have ital=0
        let mut roman = roman();
        with_modified_stat(&mut roman, |stat| set_ital_axis_value(stat, 1.0));

        let result = run_on(vec![roman]);
        assert_results_contain(
            &result,
            StatusCode::Warn,
            Some("wrong-ital-axis-value".to_string()),
        );
    }

    #[test]
    fn test_stat_ital_axis_wrong_flag_roman() {
        // Roman ital axis values must be elidable
        let mut roman = roman();
        with_modified_stat(&mut roman, |stat| {
            set_ital_axis_flags(stat, AxisValueTableFlags::empty())
        });

        let result = run_on(vec![roman]);
        assert_results_contain(
            &result,
            StatusCode::Warn,
            Some("wrong-ital-axis-flag".to_string()),
        );
    }

    #[test]
    fn test_stat_ital_axis_wrong_value_italic() {
        // Italic fonts must have ital=1
        let mut italic = italic();
        with_modified_stat(&mut italic, |stat| set_ital_axis_value(stat, 0.0));

        let result = run_on(vec![roman(), italic]);
        assert_results_contain(
            &result,
            StatusCode::Warn,
            Some("wrong-ital-axis-value".to_string()),
        );
    }

    #[test]
    fn test_stat_ital_axis_wrong_flag_italic() {
        // Italic ital axis values must not be elidable
        let mut italic = italic();
        with_modified_stat(&mut italic, |stat| {
            set_ital_axis_flags(stat, AxisValueTableFlags::ELIDABLE_AXIS_VALUE_NAME)
        });

        let result = run_on(vec![roman(), italic]);
        assert_results_contain(
            &result,
            StatusCode::Warn,
            Some("wrong-ital-axis-flag".to_string()),
        );
    }

    #[test]
    fn test_stat_ital_axis_wrong_linked_value() {
        // Roman fonts should link to the italic value 1.0
        let mut roman = roman();
        with_modified_stat(&mut roman, |stat| set_ital_axis_linked_value(stat, 0.4));

        let result = run_on(vec![roman]);
        assert_results_contain(
            &result,
            StatusCode::Warn,
            Some("wrong-ital-axis-linkedvalue".to_string()),
        );
    }

    #[test]
    fn test_stat_ital_axis_not_last() {
        let mut italic = italic();
        with_modified_stat(&mut italic, move_ital_axis_to_front);

        let result = run_on(vec![roman(), italic]);
        assert_results_contain(
            &result,
            StatusCode::Warn,
            Some("ital-axis-not-last".to_string()),
        );
    }

    #[test]
    fn test_stat_ital_axis_last_pass() {
        // The unmodified pair should not produce an ordering warning
        let result = run_on(vec![roman(), italic()]);
        let subresults = &result.as_ref().unwrap().subresults;
        assert!(!subresults
            .iter()
            .any(|s| s.code.as_deref() == Some("ital-axis-not-last")));
        assert!(!subresults
            .iter()
            .any(|s| s.code.as_deref() == Some("missing-ital-axis")));
    }

    #[test]
    fn test_stat_ital_axis_missing_ital_axis_message() {
        // The check only insists on an 'ital' axis when it can pair up a
        // roman and an italic font, so remove the axis from both.
        let mut roman = roman();
        let mut italic = italic();
        with_modified_stat(&mut roman, remove_ital_axis);
        with_modified_stat(&mut italic, remove_ital_axis);
        let result = run_on(vec![roman, italic]);
        assert_messages_contain(&result, "lacks an 'ital' axis");
    }
}

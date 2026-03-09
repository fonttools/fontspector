use fontations::skrifa::raw::{
    tables::stat::{AxisValue, AxisValueTableFlags},
    TableProvider,
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "googlefonts/STAT/opsz_not_elided",
    title = "Ensure 'opsz' axis values are not marked as elidable in STAT.",
    rationale = "
        The 'opsz' (Optical Size) axis values in the STAT table should
        not be marked as elidable. Unlike weight or width, optical size
        is an important characteristic that should always be visible to
        users so they can make informed choices about which optical size
        variant to use.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/222"
)]
fn opsz_not_elided(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let stat = f.font().stat();
    skip!(stat.is_err(), "no-stat", "Font has no STAT table.");
    let stat = stat?;

    let axes = stat.design_axes()?;
    let opsz_index = axes.iter().position(|a| a.axis_tag() == "opsz");
    skip!(
        opsz_index.is_none(),
        "no-opsz",
        "Font has no 'opsz' axis in STAT table."
    );
    let opsz_index = opsz_index.unwrap_or(0) as u16;

    let mut problems = vec![];

    if let Some(Ok(subtable)) = stat.offset_to_axis_values() {
        for val in subtable.axis_values().iter().flatten() {
            let is_opsz = match &val {
                AxisValue::Format1(v) => v.axis_index() == opsz_index,
                AxisValue::Format2(v) => v.axis_index() == opsz_index,
                AxisValue::Format3(v) => v.axis_index() == opsz_index,
                AxisValue::Format4(v) => v
                    .axis_values()
                    .iter()
                    .any(|av| av.axis_index() == opsz_index),
            };
            if is_opsz
                && val
                    .flags()
                    .contains(AxisValueTableFlags::ELIDABLE_AXIS_VALUE_NAME)
            {
                let value_str = match &val {
                    AxisValue::Format1(v) => format!("{}", v.value()),
                    AxisValue::Format2(v) => format!("{}", v.nominal_value()),
                    AxisValue::Format3(v) => format!("{}", v.value()),
                    AxisValue::Format4(_) => "Format4".to_string(),
                };
                let message = format!(
                    "STAT table 'opsz' axis value '{}' is marked as elidable. \
                     Optical size values should not be elidable.",
                    value_str
                );
                let mut status = Status::warn("elidable-opsz", &message);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "STAT".to_string(),
                    field_name: Some(format!("opsz axis value {}", value_str)),
                    actual: Some(json!("ELIDABLE_AXIS_VALUE_NAME set")),
                    expected: Some(json!("ELIDABLE_AXIS_VALUE_NAME not set")),
                    message,
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

    use fontations::write::{
        tables::stat::{AxisRecord, AxisValue, AxisValueTableFlags, Stat},
        FontBuilder,
    };
    use fontspector_checkapi::{
        codetesting::{assert_results_contain, assert_skip, run_check, test_able},
        FileTypeConvert, StatusCode, TTF,
    };

    use super::opsz_not_elided;

    #[test]
    fn test_skip_no_stat() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(opsz_not_elided, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_skip_no_opsz() {
        // Inter has STAT but no opsz axis
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(opsz_not_elided, testable);
        assert_skip(&results);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_warn_elidable_opsz() {
        use fontations::write::types::{Fixed, NameId, Tag};

        // Start from Inter and replace STAT with one that has an elidable opsz axis value
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();

        // Build a STAT table with an opsz design axis and an elidable opsz axis value
        let stat = Stat::new(
            vec![
                AxisRecord::new(Tag::new(b"wght"), NameId::new(256), 0),
                AxisRecord::new(Tag::new(b"opsz"), NameId::new(257), 1),
            ],
            vec![AxisValue::format_1(
                1, // axis_index for opsz
                AxisValueTableFlags::ELIDABLE_AXIS_VALUE_NAME,
                NameId::new(258),
                Fixed::from_f64(12.0),
            )],
            NameId::new(259),
        );

        let new_bytes = FontBuilder::new()
            .add_table(&stat)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();
        testable.contents = new_bytes;

        let results = run_check(opsz_not_elided, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("elidable-opsz".to_string()),
        );
    }
}

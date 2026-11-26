use fontations::skrifa::raw::{tables::stat::AxisValue, TableProvider};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

/// Extract (axis_index, value) pairs from a STAT axis value entry.
fn axis_value_key(val: &AxisValue) -> Vec<(u16, f32)> {
    match val {
        AxisValue::Format1(v) => vec![(v.axis_index(), v.value().to_f32())],
        AxisValue::Format2(v) => vec![(v.axis_index(), v.nominal_value().to_f32())],
        AxisValue::Format3(v) => vec![(v.axis_index(), v.value().to_f32())],
        AxisValue::Format4(v) => v
            .axis_values()
            .iter()
            .map(|av| (av.axis_index(), av.value().to_f32()))
            .collect(),
    }
}

#[check(
    id = "opentype/STAT/no_duplicate_axis_values",
    title = "Ensure STAT table has no duplicate axis value entries.",
    rationale = "
        The STAT table should not contain duplicate axis value entries.
        Duplicate entries (same axis index and value combination) can
        cause confusion in applications and may lead to unexpected
        behavior when selecting font instances.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/199"
)]
fn no_duplicate_axis_values(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let stat = f.font().stat();
    skip!(stat.is_err(), "no-stat", "Font has no STAT table.");
    let stat = stat?;
    let mut problems = vec![];

    if let Some(Ok(subtable)) = stat.offset_to_axis_values() {
        let axis_values: Vec<AxisValue> = subtable.axis_values().iter().flatten().collect();
        let axes = stat.design_axes()?;

        for (i, val_a) in axis_values.iter().enumerate() {
            let keys_a = axis_value_key(val_a);
            for val_b in axis_values.iter().skip(i + 1) {
                let keys_b = axis_value_key(val_b);
                if keys_a == keys_b {
                    let axis_names: Vec<String> = keys_a
                        .iter()
                        .map(|(idx, value)| {
                            let tag = axes
                                .get(*idx as usize)
                                .map(|a| a.axis_tag().to_string())
                                .unwrap_or_else(|| format!("axis[{}]", idx));
                            format!("{}={}", tag, value)
                        })
                        .collect();
                    let description = axis_names.join(", ");
                    let message =
                        format!("STAT table has duplicate axis value entries for: {description}");
                    let mut status = Status::fail("duplicate-axis-value", &message);
                    status.add_metadata(Metadata::TableProblem {
                        table_tag: "STAT".to_string(),
                        field_name: Some("axisValues".to_string()),
                        actual: Some(json!(description)),
                        expected: Some(json!("no duplicates")),
                        message,
                    });
                    problems.push(status);
                }
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
        codetesting::{assert_pass, assert_results_contain, assert_skip, run_check, test_able},
        FileTypeConvert, StatusCode, TTF,
    };

    use super::no_duplicate_axis_values;

    #[test]
    fn test_skip_no_stat() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(no_duplicate_axis_values, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_pass_inter_variable() {
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(no_duplicate_axis_values, testable);
        assert_pass(&results);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_fail_duplicate_axis_values() {
        use fontations::write::types::{Fixed, NameId, Tag};

        // Start from Inter and replace STAT with one that has duplicate axis value entries
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();

        // Build a STAT table with two identical Format1 entries for the same axis+value
        let stat = Stat::new(
            vec![AxisRecord::new(Tag::new(b"wght"), NameId::new(256), 0)],
            vec![
                AxisValue::format_1(
                    0,
                    AxisValueTableFlags::empty(),
                    NameId::new(257),
                    Fixed::from_f64(400.0),
                ),
                AxisValue::format_1(
                    0,
                    AxisValueTableFlags::empty(),
                    NameId::new(258),
                    Fixed::from_f64(400.0), // same axis index and value — duplicate
                ),
            ],
            NameId::new(259),
        );

        let new_bytes = FontBuilder::new()
            .add_table(&stat)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();
        testable.contents = new_bytes;

        let results = run_check(no_duplicate_axis_values, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("duplicate-axis-value".to_string()),
        );
    }
}

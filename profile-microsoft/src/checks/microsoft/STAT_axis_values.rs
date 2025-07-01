use fontations::skrifa::raw::{tables::stat::AxisValue, TableProvider};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "microsoft/STAT_axis_values",
    rationale = "
        
        Check whether STAT axis values are unique.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "STAT axis values must be unique."
)]
fn STAT_axis_values(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let Ok(stat) = f.font().stat() else {
        skip!("no-stat", "STAT table not found")
    };
    let Some(Ok(subtable)) = stat.offset_to_axis_values() else {
        return Ok(Status::just_one_warn(
            "no-axis-values",
            "STAT table has no axis values",
        ));
    };
    let format1s = subtable
        .axis_values()
        .iter()
        .flatten()
        .filter_map(|av| match av {
            AxisValue::Format1(fmt1) => Some(fmt1),
            _ => None,
        });
    let mut axis_values_format1 = vec![]; // set of (axisIndex, axisValue) tuples
    for stat_axis_value_record in format1s {
        let axis_index = stat_axis_value_record.axis_index();
        let axis_value = stat_axis_value_record.value();
        let key = (axis_index, axis_value);
        if axis_values_format1.contains(&key) {
            problems.push(Status::fail(
                "not-unique",
                &format!("axis value {axis_value} (format 1) for axis #{axis_index} is not unique"),
            ));
        } else {
            axis_values_format1.push(key);
        }
    }
    return_result(problems)
}

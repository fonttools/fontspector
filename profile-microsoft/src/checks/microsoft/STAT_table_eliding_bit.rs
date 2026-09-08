use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::raw::{tables::stat::AxisValueTableFlags, TableProvider};

#[check(
    id = "microsoft/STAT_table_eliding_bit",
    rationale = "
        
        Validate STAT table eliding bit.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Validate STAT table eliding bit"
)]
fn STAT_table_eliding_bit(t: &Testable, _context: &Context) -> CheckFnResult {
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
    for value in subtable.axis_values().iter().flatten() {
        let value_name_id = value.value_name_id();
        let flags = value.flags();
        if let Some(value_name) = f.get_best_name(&[value_name_id]) {
            if value_name == "Regular"
                && !flags.contains(AxisValueTableFlags::ELIDABLE_AXIS_VALUE_NAME)
            {
                problems.push(Status::fail(
                    "not-elided",
                    &format!(
                        "axis value {value_name} (format {}) is not elided",
                        value.format()
                    ),
                ));
            }
        } else {
            problems.push(Status::fail(
                "missing-name",
                &format!("axis value {value_name_id} is missing a name"),
            ));
        }
    }
    return_result(problems)
}

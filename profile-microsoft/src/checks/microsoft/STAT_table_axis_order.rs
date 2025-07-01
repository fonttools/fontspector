use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "microsoft/STAT_table_axis_order",
    rationale = "
        
        Validate STAT table axisOrder.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "STAT table axis order."
)]
fn STAT_table_axis_order(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let Ok(stat) = f.font().stat() else {
        skip!("no-stat", "STAT table not found")
    };

    let mut axes = stat
        .design_axes()?
        .iter()
        .map(|axis| (axis.axis_tag().to_string(), axis.axis_ordering()))
        .collect::<Vec<_>>();
    axes.sort_by_key(|(_, ordering)| -(*ordering as i16)); // sort in reverse order

    const AXIS_ORDER_REVERSED: [&str; 5] = ["ital", "slnt", "wdth", "wght", "opsz"];

    let axes = axes.iter().map(|(tag, _)| tag.as_str()).collect::<Vec<_>>();

    let mut index = 0;
    for axis in AXIS_ORDER_REVERSED {
        if let Some(pos) = axes.iter().position(|&x| x == axis) {
            if pos != index {
                return Ok(Status::just_one_fail(
                    "axisOrder",
                    &format!("STAT table axisOrder for {axis} is not valid"),
                ));
            }
            index += 1;
        }
    }
    Ok(Status::just_one_pass())
}

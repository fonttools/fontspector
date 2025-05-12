use std::collections::HashSet;

use fontations::skrifa::raw::{tables::stat::AxisValue, TableProvider};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use itertools::Itertools;

#[check(
    id = "adobefonts/STAT_strings",
    rationale = "
        
        In the STAT table, the \"Italic\" keyword must not be used on AxisValues
        for variation axes other than 'ital' or 'slnt'. This is a more lenient
        implementation of googlefonts/STAT_strings which allows \"Italic\"
        only for the 'ital' axis.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2863",
    title = "Check correctness of STAT table strings"
)]
fn STAT_strings(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(!f.has_table(b"STAT"), "no-stat", "No STAT table.");
    let stat = f.font().stat()?;
    let axes = stat.design_axes()?;
    let ital_pos = axes
        .iter()
        .position(|axis| axis.axis_tag() == "ital")
        .map(|x| x as u16);
    let slnt_pos = axes
        .iter()
        .position(|axis| axis.axis_tag() == "slnt")
        .map(|x| x as u16);
    let mut name_ids = HashSet::new();
    if let Some(Ok(subtable)) = stat.offset_to_axis_values() {
        for axis_value in subtable.axis_values().iter().flatten() {
            match axis_value {
                AxisValue::Format1(v) => {
                    if Some(v.axis_index()) != ital_pos && Some(v.axis_index()) != slnt_pos {
                        name_ids.insert(v.value_name_id());
                    }
                }
                AxisValue::Format2(v) => {
                    if Some(v.axis_index()) != ital_pos && Some(v.axis_index()) != slnt_pos {
                        name_ids.insert(v.value_name_id());
                    }
                }
                AxisValue::Format3(v) => {
                    if Some(v.axis_index()) != ital_pos && Some(v.axis_index()) != slnt_pos {
                        name_ids.insert(v.value_name_id());
                    }
                }
                AxisValue::Format4(v) => {
                    for axis_value in v.axis_values() {
                        if Some(axis_value.axis_index()) != ital_pos
                            && Some(axis_value.axis_index()) != slnt_pos
                        {
                            name_ids.insert(v.value_name_id());
                        }
                    }
                }
            }
        }
    }
    let problems = name_ids
        .into_iter()
        .filter(|id| {
            f.get_name_entry_strings(*id)
                .any(|s| s.to_lowercase().contains("italic"))
        })
        .map(|id| id.to_string())
        .sorted()
        .join(", ");
    if !problems.is_empty() {
        Ok(Status::just_one_fail(
            "bad-italic",
            &format!("The following AxisValue entries on the STAT table should not contain \"Italic\": {}", problems),
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}

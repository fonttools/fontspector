use std::collections::BTreeMap;

use fontations::skrifa::raw::{tables::stat::AxisValue, TableProvider};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "microsoft/fvar_STAT_axis_ranges",
    rationale = "
        
        Check fvar named instance axis values lie within a single STAT axis range.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Requirements for named instances and STAT axis ranges."
)]
fn fvar_STAT_axis_ranges(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let Ok(stat) = f.font().stat() else {
        skip!("no-stat", "STAT table not found")
    };
    let stat_axis_tags = stat
        .design_axes()?
        .iter()
        .map(|axis| axis.axis_tag().to_string())
        .collect::<Vec<_>>();
    let Some(Ok(subtable)) = stat.offset_to_axis_values() else {
        return Ok(Status::just_one_warn(
            "no-axis-values",
            "STAT table has no axis values",
        ));
    };
    for (_name, coordinates) in f.named_instances() {
        let mut found_in_format4 = false;
        let format4s = subtable
            .axis_values()
            .iter()
            .flatten()
            .filter_map(|av| match av {
                AxisValue::Format4(fmt4) => Some(fmt4),
                _ => None,
            });
        for stat_axis_value_record in format4s {
            let stat_coord_set = stat_axis_value_record
                .axis_values()
                .iter()
                .filter_map(|av| {
                    let axis_index = av.axis_index();
                    if axis_index >= stat_axis_tags.len() as u16 {
                        problems.push(Status::fail(
                            "bad-axis-index",
                            &format!(
                                "axis index {} (format 4) is greater than STAT axis count {}",
                                axis_index,
                                stat_axis_tags.len()
                            ),
                        ));
                        return None;
                    }
                    #[allow(clippy::indexing_slicing)] // We know it's in bounds
                    let stat_axis = &stat_axis_tags[axis_index as usize];
                    Some((stat_axis.to_string(), av.value().to_f32()))
                })
                .collect::<BTreeMap<_, _>>();
            if coordinates == stat_coord_set {
                found_in_format4 = true;
                break;
            }
        }
        if found_in_format4 {
            continue;
        }
        for (instance_axis, &instance_value) in coordinates.iter() {
            let mut found_instance_axis = false;
            // for each axis value record in STAT
            let axis_values = subtable.axis_values().iter().flatten();

            for stat_axis_value_record in axis_values {
                match stat_axis_value_record {
                    AxisValue::Format1(av1) => {
                        if av1.axis_index() > stat_axis_tags.len() as u16 {
                            problems.push(Status::fail(
                                "bad-axis-index",
                                &format!(
                                    "axis index {} (format 1) is greater than STAT axis count {}",
                                    av1.axis_index(),
                                    stat_axis_tags.len()
                                ),
                            ));
                        } else {
                            #[allow(clippy::indexing_slicing)] // We know it's in bounds
                            let stat_axis = &stat_axis_tags[av1.axis_index() as usize];
                            if instance_axis == stat_axis && instance_value == av1.value().to_f32()
                            {
                                if found_instance_axis {
                                    problems.push(Status::fail(
                                        "axis-value-non-unique",
                                        &format!(
                                            "axis value {instance_value} (format 1) for axis {instance_axis} is not unique"
                                        ),
                                    ));
                                }
                                found_instance_axis = true;
                            }
                        }
                    }
                    // When Skrifa 0.31 is released we won't need this repetition
                    AxisValue::Format3(av3) => {
                        if av3.axis_index() > stat_axis_tags.len() as u16 {
                            problems.push(Status::fail(
                                "bad-axis-index",
                                &format!(
                                    "axis index {} (format 3) is greater than STAT axis count {}",
                                    av3.axis_index(),
                                    stat_axis_tags.len()
                                ),
                            ));
                        } else {
                            #[allow(clippy::indexing_slicing)] // We know it's in bounds
                            let stat_axis = &stat_axis_tags[av3.axis_index() as usize];
                            if instance_axis == stat_axis && instance_value == av3.value().to_f32()
                            {
                                if found_instance_axis {
                                    problems.push(Status::fail(
                                        "non-unique",
                                        &format!(
                                            "axis value {instance_value} (format 3) for axis {instance_axis} is not unique"
                                        ),
                                    ));
                                }
                                found_instance_axis = true;
                            }
                        }
                    }
                    AxisValue::Format2(av2) => {
                        if av2.axis_index() > stat_axis_tags.len() as u16 {
                            problems.push(Status::fail(
                                "bad-axis-index",
                                &format!(
                                    "axis index {} (format 2) is greater than STAT axis count {}",
                                    av2.axis_index(),
                                    stat_axis_tags.len()
                                ),
                            ));
                        } else {
                            #[allow(clippy::indexing_slicing)] // We know it's in bounds
                            let stat_axis = &stat_axis_tags[av2.axis_index() as usize];
                            if instance_axis == stat_axis
                                && instance_value == av2.nominal_value().to_f32()
                            {
                                if found_instance_axis {
                                    problems.push(Status::fail(
                                        "non-unique",
                                        &format!(
                                            "axis value {instance_value} (format 2) for axis {instance_axis} is not unique"
                                        ),
                                    ));
                                }
                                found_instance_axis = true;
                            }
                        }
                    }
                    AxisValue::Format4(_) => {}
                }
            }
            if !found_instance_axis {
                problems.push(Status::fail(
                    "axis-value-not-found",
                    &format!(
                        "axis value {instance_value} for axis {instance_axis} not found in STAT table"
                    ),
                ));
            }
        }
    }
    return_result(problems)
}

use std::collections::HashMap;

use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "varfont/consistent_axes",
    rationale = "
        In order to facilitate the construction of intuitive and friendly user
        interfaces, all variable font files in a given family should have the same set
        of variation axes. Also, each axis must have a consistent setting of min/max
        value ranges accross all the files.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2810",
    title = "Ensure that all variable font files have the same set of axes and axis ranges.",
    implementation = "all"
)]
fn consistent_axes(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let ttfs = TTF.from_collection(c);
    let fonts = ttfs
        .iter()
        .filter(|f| f.is_variable_font())
        .collect::<Vec<_>>();
    let mut problems = vec![];
    skip!(fonts.len() < 2, "no-siblings", "No sibling fonts found");
    let mut reference_ranges = HashMap::new();
    for font in fonts.iter() {
        for axis in font.font().axes().iter() {
            reference_ranges
                .entry(axis.tag())
                .or_insert((axis.min_value(), axis.max_value()));
        }
    }
    for font in fonts.iter() {
        for (axis, &(a_min, a_max)) in reference_ranges.iter() {
            if let Some(found_axis) = font.font().axes().iter().find(|a| a.tag() == *axis) {
                if found_axis.min_value() != a_min || found_axis.max_value() != a_max {
                    let message = format!(
                        "Font {} has inconsistent range for axis {}: expected [{}, {}], found [{}, {}]",
                        font.filename.to_str().unwrap_or("Unknown font"),
                        axis,
                        a_min,
                        a_max,
                        found_axis.min_value(),
                        found_axis.max_value()
                    );
                    let mut status = Status::fail("inconsistent-axis-range", &message);
                    status.add_metadata(Metadata::TableProblem {
                        table_tag: "fvar".to_string(),
                        field_name: Some(format!("axis {}", axis)),
                        actual: Some(
                            json!({ "min": found_axis.min_value(), "max": found_axis.max_value() }),
                        ),
                        expected: Some(json!({ "min": a_min, "max": a_max })),
                        message,
                    });
                    problems.push(status);
                }
            } else {
                let message = format!(
                    "Font {} is missing axis {}",
                    font.filename.to_str().unwrap_or("Unknown font"),
                    axis
                );
                let mut status = Status::fail("missing-axis", &message);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "fvar".to_string(),
                    field_name: Some(format!("axis {}", axis)),
                    actual: None,
                    expected: Some(json!({ "axis": axis.to_string() })),
                    message,
                });
                problems.push(status);
            }
        }
    }
    return_result(problems)
}

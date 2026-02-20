use fontations::skrifa::{
    raw::{ReadError, TableProvider},
    setting::{Setting, VariationSetting},
    GlyphId, Tag,
};
use fontdrasil::coords::{NormalizedCoord, NormalizedLocation};
use fontspector_checkapi::{
    pens::BezGlyph, prelude::*, skip, testfont, FileTypeConvert, Metadata, TestFont,
};
use interpolatable::{run_tests, Problem, ProblemDetails};
use serde_json::json;
use std::collections::HashMap;

fn denormalize_location(
    font: &TestFont,
    tuple: &[f32],
) -> Result<Vec<VariationSetting>, FontspectorError> {
    let all_axes = font.fontdrasil_axes()?.ok_or(FontspectorError::General(
        "Axes went away between function calls".to_string(),
    ))?;
    let loc: NormalizedLocation = all_axes
        .iter()
        .zip(tuple)
        .filter(|&(_axis, peak)| *peak != 0.0)
        .map(|(axis, peak)| (axis.tag, NormalizedCoord::new(*peak as f64)))
        .collect();
    let user = loc.to_user(&all_axes);
    // And now back to skrifa
    Ok(user
        .iter()
        .map(|(tag, value)| VariationSetting {
            selector: Tag::new(&tag.to_be_bytes()), // different version madness
            value: value.to_f64() as f32,
        })
        .collect())
}

fn glyph_variations(
    font: &TestFont,
    gid: GlyphId,
) -> Result<Vec<Vec<VariationSetting>>, ReadError> {
    Ok(font
        .font()
        .gvar()?
        .glyph_variation_data(gid)?
        .map_or_else(Vec::new, |data| {
            data.tuples()
                .flat_map(|t| {
                    let tuple: Vec<f32> =
                        t.peak().values.iter().map(|v| v.get().to_f32()).collect();
                    denormalize_location(font, &tuple)
                })
                .collect()
        }))
}

fn variation_settings_to_hashmap(settings: &[VariationSetting]) -> HashMap<String, f32> {
    settings
        .iter()
        .map(|s| (s.selector.to_string(), s.value))
        .collect()
}

fn problem_report(p: &Problem) -> String {
    match &p.details {
        ProblemDetails::PathCount { count_1, count_2 } => {
            format!(
                "Path count mismatch: {} in {} vs {} in {}",
                count_1, p.master_1_name, count_2, p.master_2_name
            )
        }
        ProblemDetails::NodeCount { count_1, count_2 } => {
            format!(
                "Node count mismatch: {} in {} vs {} in {}",
                count_1, p.master_1_name, count_2, p.master_2_name
            )
        }
        ProblemDetails::NodeIncompatibility {
            is_control_1,
            is_control_2,
        } => format!(
            "Incompatible nodes: mismatch: {} is {} in {} vs {} in {}",
            p.node.unwrap_or(0),
            (if *is_control_1 {
                "off-curve"
            } else {
                "on-curve"
            }),
            p.master_1_name,
            (if *is_control_2 {
                "off-curve"
            } else {
                "on-curve"
            }),
            p.master_2_name,
        ),
        ProblemDetails::ContourOrder { order_1, order_2 } => format!(
            "Contour order mismatch: {:?} in {} vs {:?} in {}",
            order_1, p.master_1_name, order_2, p.master_2_name
        ),
        ProblemDetails::WrongStartPoint {
            proposed_point,
            reverse,
        } => format!(
            "Wrong start point: contour {} should start at {} in {}{}",
            p.contour.unwrap_or(0),
            proposed_point,
            p.master_2_name,
            if *reverse {
                " (and contour should be reversed)"
            } else {
                ""
            }
        ),
        ProblemDetails::Overweight {
            value_1: _,
            value_2: _,
        } => {
            format!(
                "Contour {} becomes overweight in {} compared to {}",
                p.contour.unwrap_or(0),
                p.master_2_name,
                p.master_1_name
            )
        }
        ProblemDetails::Underweight {
            value_1: _,
            value_2: _,
        } => {
            format!(
                "Contour {} becomes underweight in {} compared to {}",
                p.contour.unwrap_or(0),
                p.master_2_name,
                p.master_1_name
            )
        }
        ProblemDetails::Kink => format!(
            "Kink in contour {} at node {}",
            p.contour.unwrap_or(0),
            p.node.unwrap_or(0)
        ),
    }
}
#[check(
    id = "interpolation_issues",
    rationale = "
        When creating a variable font, the designer must make sure that corresponding
        paths have the same start points across masters, as well as that corresponding
        component shapes are placed in the same order within a glyph across masters.
        If this is not done, the glyph will not interpolate correctly.

        Here we check for the presence of potential interpolation errors using the
        interpolatable crate.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3930",
    title = "Detect any interpolation issues in the font."
)]
fn interpolation_issues(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let font = f.font();
    let upem = font.head()?.units_per_em();
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not variable"
    );

    let mut all_statuses: Vec<Status> = vec![];
    let mut locations: Vec<Vec<VariationSetting>> = vec![vec![]];
    for gid in f.all_glyphs() {
        let glyphname = f.glyph_name_for_id_synthesise(gid);
        let mut default_glyph = interpolatable::Glyph::new_from_font(&font, gid, &[]).ok_or(
            FontspectorError::General(format!("Can't convert glyph {glyphname}")),
        )?;
        default_glyph.master_name = "default".to_string();
        default_glyph.master_index = 0;

        if let Ok(variations) = glyph_variations(&f, gid) {
            for variation in variations {
                let mut glyph_instance =
                    interpolatable::Glyph::new_from_font(&font, gid, &variation).ok_or(
                        FontspectorError::General(format!("Can't convert glyph {glyphname}")),
                    )?;
                glyph_instance.master_name = variation
                    .iter()
                    .map(|v| format!("{}={}", v.selector, v.value))
                    .collect::<Vec<_>>()
                    .join(",");
                if !locations.contains(&variation) {
                    locations.push(variation.clone());
                }
                glyph_instance.master_index =
                    locations.iter().position(|x| x == &variation).ok_or(
                        FontspectorError::General("Can't find master index".to_string()),
                    )?;

                let problems = run_tests(&default_glyph, &glyph_instance, None, None, Some(upem));
                if !problems.is_empty() {
                    let userspace_location_map = variation_settings_to_hashmap(&variation);

                    for problem in problems {
                        let message = problem_report(&problem);
                        let mut metadata_position: Option<(f32, f32)> = None;
                        let mut metadata_actual: Option<serde_json::Value> = None;
                        let mut metadata_expected: Option<serde_json::Value> = None;

                        match &problem.details {
                            ProblemDetails::PathCount { count_1, count_2 } => {
                                metadata_actual = Some(json!({ "path_count_master1": count_1 }));
                                metadata_expected = Some(json!({ "path_count_master2": count_2 }));
                            }
                            ProblemDetails::NodeCount { count_1, count_2 } => {
                                metadata_actual = Some(json!({ "node_count_master1": count_1 }));
                                metadata_expected = Some(json!({ "node_count_master2": count_2 }));
                            }
                            ProblemDetails::NodeIncompatibility {
                                is_control_1,
                                is_control_2,
                            } => {
                                if let Some(node_idx) = problem.node {
                                    // Attempt to get node coordinates if possible, this is complex and might require more context
                                    // For now, we'll just report the node index
                                    metadata_position = None; // Placeholder, actual coordinates would be better
                                    metadata_actual = Some(
                                        json!({ "node_index": node_idx, "is_control_master1": is_control_1 }),
                                    );
                                    metadata_expected =
                                        Some(json!({ "is_control_master2": is_control_2 }));
                                }
                            }
                            ProblemDetails::ContourOrder { order_1, order_2 } => {
                                metadata_actual = Some(json!({ "contour_order_master1": order_1 }));
                                metadata_expected =
                                    Some(json!({ "contour_order_master2": order_2 }));
                            }
                            ProblemDetails::WrongStartPoint {
                                proposed_point,
                                reverse,
                            } => {
                                if let Some(contour_idx) = problem.contour {
                                    // Position could be the proposed_point if it's a coordinate
                                    metadata_position = get_point_by_index_at_location(
                                        &f,
                                        gid,
                                        contour_idx,
                                        *proposed_point,
                                        &variation,
                                    );
                                    metadata_actual = Some(json!({ "contour_index": contour_idx }));
                                    metadata_expected = Some(
                                        json!({ "proposed_start_point": proposed_point, "reverse_contour": reverse }),
                                    );
                                }
                            }
                            ProblemDetails::Overweight { value_1, value_2 } => {
                                if let Some(contour_idx) = problem.contour {
                                    metadata_actual = Some(
                                        json!({ "contour_index": contour_idx, "value_master1": value_1 }),
                                    );
                                    metadata_expected = Some(json!({ "value_master2": value_2 }));
                                }
                            }
                            ProblemDetails::Underweight { value_1, value_2 } => {
                                if let Some(contour_idx) = problem.contour {
                                    metadata_actual = Some(
                                        json!({ "contour_index": contour_idx, "value_master1": value_1 }),
                                    );
                                    metadata_expected = Some(json!({ "value_master2": value_2 }));
                                }
                            }
                            ProblemDetails::Kink => {
                                if let Some(contour_idx) = problem.contour {
                                    if let Some(node_idx) = problem.node {
                                        metadata_position = get_point_by_index_at_location(
                                            &f,
                                            gid,
                                            contour_idx,
                                            node_idx,
                                            &variation,
                                        );
                                        metadata_actual = Some(
                                            json!({ "contour_index": contour_idx, "node_index": node_idx }),
                                        );
                                    }
                                }
                            }
                        }

                        let mut status = Status::warn(
                            "interpolation-issue",
                            &format!("Interpolation issue in {}: {}", glyphname, message),
                        );
                        status.add_metadata(Metadata::GlyphProblem {
                            glyph_name: glyphname.clone(),
                            glyph_id: gid.to_u32(),
                            position: metadata_position,
                            message: message.clone(),
                            actual: metadata_actual,
                            expected: metadata_expected,
                            userspace_location: Some(userspace_location_map.clone()),
                        });
                        all_statuses.push(status);
                    }
                }
            }
        }
    }
    return_result(all_statuses)
}

fn get_point_by_index_at_location(
    font: &TestFont,
    gid: GlyphId,
    contour_idx: usize,
    proposed_point: usize,
    variation: &[Setting<f32>],
) -> Option<(f32, f32)> {
    // Get the outline from skrifa
    let mut bezglyph = BezGlyph::default();
    font.draw_glyph(gid, &mut bezglyph, variation).ok()?;
    // Extract the point from the bezglyph
    let paths = bezglyph.0;
    let contour = paths.get(contour_idx)?;
    let points = contour
        .iter()
        .nth(proposed_point)?
        .end_point() // Or maybe start point?!
        .map(|p| (p.x as f32, p.y as f32));
    points
}

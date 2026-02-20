use std::collections::BTreeSet;

use fontations::{
    skrifa::{
        metrics::GlyphMetrics,
        prelude::{LocationRef, NormalizedCoord, Size},
        MetadataProvider,
    },
    types::GlyphId,
};
use fontdrasil::types::Axes;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata, TestFont};
use hashbrown::HashMap;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use serde_json::json;

fn denormalize_location(normalized_coords: &[NormalizedCoord], axes: &Axes) -> Vec<(String, f64)> {
    normalized_coords
        .iter()
        .zip(axes.iter())
        .map(|(coord, axis)| {
            (
                axis.tag.to_string(),
                fontdrasil::coords::NormalizedCoord::new(coord.to_f32() as f64)
                    .to_user(&axis.converter)
                    .to_f64(),
            )
        })
        .collect()
}

// Stolen from fontheight.
//
// Gets all combinations of axis coordinates seen in named instances, axis
// extremes, and the default location.
//
// Note: the number of [`Location`]s this method returns scales
// exponentially with the number of axes.
pub fn interesting_locations(t: &TestFont) -> Vec<Vec<NormalizedCoord>> {
    let mut axis_coords = vec![BTreeSet::<OrderedFloat<f32>>::new(); t.font().axes().len()];

    t.font()
        .named_instances()
        .iter()
        .flat_map(|instance| instance.user_coords().enumerate())
        .for_each(|(axis, coord)| {
            #[allow(clippy::indexing_slicing)]
            // We initialized axis_coords with the right number of axes
            axis_coords[axis].insert(coord.into());
        });

    t.font().axes().iter().for_each(|axis| {
        #[allow(clippy::indexing_slicing)]
        // We initialized axis_coords with the right number of axes
        axis_coords[axis.index()].extend(&[
            axis.default_value().into(),
            axis.min_value().into(),
            axis.max_value().into(),
        ]);
    });

    let axes = t.font().axes();
    axis_coords
        .iter()
        .multi_cartesian_product()
        .map(|coords| {
            coords
                .into_iter()
                .zip(axes.iter())
                .map(|(coord, axis)| axis.normalize(**coord))
                .collect()
        })
        .collect()
}

fn detect_outliers(
    some_measurement: &HashMap<GlyphId, Vec<f32>>,
    z_score_threshold: f32,
    code: &str,
    measurement_name: &str,
    font: &TestFont,
    location_names: &[String],
) -> Vec<Status> {
    let mut problems = vec![];

    // Determine the number of locations by looking at any glyph's variations
    let num_locations = some_measurement
        .values()
        .next()
        .map(|v| v.len())
        .unwrap_or(0);

    if num_locations == 0 {
        return problems;
    }

    // For each location, compute mean and standard deviation across all glyphs
    let mut location_stats: Vec<(f32, f32)> = vec![(0.0, 0.0); num_locations];

    for (loc, stats) in location_stats.iter_mut().enumerate() {
        let values: Vec<f32> = some_measurement
            .values()
            .filter_map(|variations| variations.get(loc).copied())
            .collect();

        if values.is_empty() {
            continue;
        }

        let mean = values.iter().sum::<f32>() / values.len() as f32;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / values.len() as f32;
        let std_dev = variance.sqrt();

        *stats = (mean, std_dev);
    }

    // Check each glyph at each location for Z-score outliers
    let mut problems_by_glyph: HashMap<GlyphId, Vec<(usize, f32)>> = HashMap::new();
    for (glyph, variations) in some_measurement.iter() {
        for (loc, variation) in variations.iter().enumerate() {
            let (mean, std_dev) = *location_stats.get(loc).unwrap_or(&(0.0, 0.0));

            let z_score = if std_dev > 0.0 {
                (variation - mean).abs() / std_dev
            } else {
                0.0
            };

            log::debug!(
                "Glyph {:?} has variation {:.2}% in {} at location {} (mean {:.2}%, std dev {:.2}%, Z-score {:.2})",
                font.glyph_name_for_id(*glyph),
                variation * 100.0,
                measurement_name,
                &location_names.get(loc).unwrap_or(&"<unknown>".to_string()),
                mean * 100.0,
                std_dev * 100.0,
                z_score
            );

            if z_score > z_score_threshold {
                problems_by_glyph
                    .entry(*glyph)
                    .or_insert_with(Vec::new)
                    .push((loc, z_score));
            }
        }
    }

    for (glyph, outliers) in problems_by_glyph {
        let max_zscore = outliers.iter().map(|(_, z)| *z).fold(0.0, f32::max);
        let location_descriptions: Vec<String> = outliers
            .iter()
            .map(|(loc, _)| {
                location_names
                    .get(*loc)
                    .unwrap_or(&"<unknown>".to_string())
                    .to_string()
            })
            .collect();

        let message = format!(
            "Glyph {:?} has suspiciously high variation (z-score {:.2}) in {} at locations:\n    {}",
            font.glyph_name_for_id_synthesise(glyph),
            max_zscore,
            measurement_name,
            location_descriptions.join("\n    ")
        );
        let mut status = Status::warn(code, &message);
        for (loc, z_score) in outliers {
            let userspace_location = location_names
                .get(loc)
                .and_then(|name| parse_location_map(name));
            status.add_metadata(Metadata::GlyphProblem {
                glyph_name: font.glyph_name_for_id_synthesise(glyph),
                glyph_id: glyph.to_u32(),
                userspace_location,
                position: None,
                actual: Some(json!({ "z_score": z_score, "measurement": measurement_name })),
                expected: Some(json!({ "z_score_max": z_score_threshold })),
                message: format!("Suspicious variation in {}", measurement_name),
            });
        }
        problems.push(status);
    }
    problems
}

fn parse_location_map(location_name: &str) -> Option<std::collections::HashMap<String, f32>> {
    if location_name == "<unknown>" {
        return None;
    }
    let mut map = std::collections::HashMap::new();
    for part in location_name.split(", ") {
        let mut iter = part.split('=');
        let tag = iter.next()?;
        let value = iter.next()?;
        if iter.next().is_some() {
            return None;
        }
        let parsed = value.parse::<f32>().ok()?;
        map.insert(tag.to_string(), parsed);
    }
    Some(map)
}

fn right_side_bearing(glyph: GlyphId, metrics: &GlyphMetrics) -> f32 {
    let aw = metrics.advance_width(glyph).unwrap_or_default();
    if let Some(bounds) = metrics.bounds(glyph) {
        aw - bounds.x_max
    } else {
        aw
    }
}

#[check(
    id = "suspicious_sidebearings",
    rationale = "
        Glyphs in variable fonts should vary across the designspace in a
        consistent way. If there are outliers the advance width, left
        or right sidebearings of glyphs at certain locations in the designspace,
        this may indicate design issue with the font.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/557",
    title = "Ensure variable fonts have relatively consistent sidebearings."
)]
fn suspicious_sidebearings(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font"
    );
    let mut problems = vec![];

    let locs = interesting_locations(&f);
    let axes = f.fontdrasil_axes()?.unwrap_or_default();
    let location_names: Vec<String> = locs
        .iter()
        .map(|loc| {
            denormalize_location(loc, &axes)
                .iter()
                .map(|(axis, coord)| format!("{}={:.2}", axis, coord))
                .join(", ")
        })
        .collect();

    let mut left_sidebearing = HashMap::new();
    let mut right_sidebearing = HashMap::new();
    let mut advance_width = HashMap::new();
    let metrics_at_default = GlyphMetrics::new(&f.font(), Size::unscaled(), LocationRef::default());
    let lsb_at_default: HashMap<GlyphId, f32> = f
        .all_glyphs()
        .map(|gid| {
            (
                gid,
                metrics_at_default
                    .left_side_bearing(gid)
                    .unwrap_or_default(),
            )
        })
        .collect();
    let rsb_at_default: HashMap<GlyphId, f32> = f
        .all_glyphs()
        .map(|gid| (gid, right_side_bearing(gid, &metrics_at_default)))
        .collect();
    let advance_width_at_default: HashMap<GlyphId, f32> = f
        .all_glyphs()
        .map(|gid| {
            (
                gid,
                metrics_at_default.advance_width(gid).unwrap_or_default(),
            )
        })
        .collect();
    for loc in locs.iter() {
        let metrics = GlyphMetrics::new(&f.font(), Size::unscaled(), LocationRef::new(loc));
        for glyph in f.all_glyphs() {
            let lsb = metrics.left_side_bearing(glyph).unwrap_or_default();
            let rsb = right_side_bearing(glyph, &metrics);
            let aw = metrics.advance_width(glyph).unwrap_or_default();
            let default_lsb = lsb_at_default.get(&glyph).copied().unwrap_or_default();
            let default_rsb = rsb_at_default.get(&glyph).copied().unwrap_or_default();
            let default_aw = advance_width_at_default
                .get(&glyph)
                .copied()
                .unwrap_or_default();
            let lsb_variation = (lsb - default_lsb).abs();
            let rsb_variation = (rsb - default_rsb).abs();
            let aw_variation = (aw - default_aw).abs();
            left_sidebearing
                .entry(glyph)
                .or_insert_with(Vec::new)
                .push(lsb_variation);
            right_sidebearing
                .entry(glyph)
                .or_insert_with(Vec::new)
                .push(rsb_variation);
            advance_width
                .entry(glyph)
                .or_insert_with(Vec::new)
                .push(aw_variation);
        }
    }
    const Z_SCORE_THRESHOLD: f32 = 10.0;
    // const Z_SCORE_THRESHOLD: f32 = 2.5;
    problems.extend(detect_outliers(
        &left_sidebearing,
        Z_SCORE_THRESHOLD,
        "large-lsb-variation",
        "left sidebearings",
        &f,
        &location_names,
    ));
    problems.extend(detect_outliers(
        &right_sidebearing,
        Z_SCORE_THRESHOLD,
        "large-rsb-variation",
        "right sidebearings",
        &f,
        &location_names,
    ));
    problems.extend(detect_outliers(
        &advance_width,
        Z_SCORE_THRESHOLD,
        "large-aw-variation",
        "advance widths",
        &f,
        &location_names,
    ));

    return_result(problems)
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, run_check, test_able,
    };

    use fontspector_checkapi::StatusCode;

    use super::suspicious_sidebearings;

    #[test]
    fn test_suspicious_sidebearings() {
        // Akshar is good
        let testable = test_able("akshar/Akshar[wght].ttf");
        let results = run_check(suspicious_sidebearings, testable);
        assert_pass(&results);
        // Should skip a non-variable
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let results = run_check(suspicious_sidebearings, testable);
        assert_results_contain(&results, StatusCode::Skip, Some("not-variable".to_string()));
        // Inter has all kinds of problems
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(suspicious_sidebearings, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("large-rsb-variation".to_string()),
        );
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("large-aw-variation".to_string()),
        );
    }
}

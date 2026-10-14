use fontspector_checkapi::{prelude::*, FileTypeConvert, Metadata, TestFont};
use itertools::Itertools;
use serde_json::json;
use skrifa::{
    instance::{Location, NormalizedCoord, Size},
    outline::{DrawSettings, OutlinePen},
    raw::TableProvider,
    GlyphId, MetadataProvider,
};

use super::designspace::denormalize_location;

/// A pen which records the lowest and highest Y coordinate it is given,
/// including off-curve control points. This matches how the glyf table's
/// yMin and yMax are defined.
#[derive(Debug, Default)]
struct YBoundsPen {
    ymin: Option<f32>,
    ymax: Option<f32>,
}

impl YBoundsPen {
    fn update(&mut self, y: f32) {
        self.ymin = Some(self.ymin.map_or(y, |v| v.min(y)));
        self.ymax = Some(self.ymax.map_or(y, |v| v.max(y)));
    }
}

impl OutlinePen for YBoundsPen {
    fn move_to(&mut self, _x: f32, y: f32) {
        self.update(y);
    }
    fn line_to(&mut self, _x: f32, y: f32) {
        self.update(y);
    }
    fn quad_to(&mut self, _cx0: f32, cy0: f32, _x: f32, y: f32) {
        self.update(cy0);
        self.update(y);
    }
    fn curve_to(&mut self, _cx0: f32, cy0: f32, _cx1: f32, cy1: f32, _x: f32, y: f32) {
        self.update(cy0);
        self.update(cy1);
        self.update(y);
    }
    fn close(&mut self) {}
}

/// Format a user-space axis value, hiding float noise from the
/// normalized-to-user conversion.
fn format_axis_value(value: f32) -> String {
    let rounded = (value * 100.0).round() / 100.0;
    if (rounded - rounded.round()).abs() < f32::EPSILON {
        format!("{}", rounded.round() as i64)
    } else {
        format!("{rounded}")
    }
}

/// Where an extreme Y value was found.
#[derive(Debug, Clone)]
struct Extreme {
    glyph: GlyphId,
    /// Normalized designspace coordinates, one per axis. Empty for static fonts.
    location: Vec<f32>,
}

#[derive(Debug, Default)]
struct Metrics {
    ymin: i16,
    ymax: i16,
    lowest: Option<Extreme>,
    highest: Option<Extreme>,
}

impl Metrics {
    fn describe(&self, fonts: &[TestFont], extreme: &Option<Extreme>, which: &str) -> String {
        let Some(extreme) = extreme else {
            return String::new();
        };
        // Any font in the family can name the glyph; use the first that
        // actually contains it.
        let Some(font) = fonts
            .iter()
            .find(|f| f.all_glyphs().any(|g| g == extreme.glyph))
        else {
            return String::new();
        };
        let name = font.glyph_name_for_id_synthesise(extreme.glyph);
        let location = if extreme.location.iter().any(|v| *v != 0.0) {
            denormalize_location(font, &extreme.location)
                .ok()
                .map(|settings| {
                    settings
                        .iter()
                        .map(|s| format!("{}={}", s.selector, format_axis_value(s.value)))
                        .join(", ")
                })
                .filter(|s| !s.is_empty())
        } else {
            None
        };
        match location {
            Some(location) => format!(" The {which} point is in glyph '{name}' at {location}."),
            None => format!(" The {which} point is in glyph '{name}'."),
        }
    }
}

/// The normalized designspace locations at which a glyph should be measured.
///
/// For a static font this is just the default location. For a variable
/// font we add the peak of every gvar tuple for this glyph, since the
/// extremes of a piecewise-linear interpolation sit at those peaks.
fn glyph_locations(font: &TestFont, gid: GlyphId, axis_count: usize) -> Vec<Vec<f32>> {
    let mut locations = vec![vec![0.0_f32; axis_count]];
    if axis_count == 0 {
        return locations;
    }
    let Ok(gvar) = font.font().gvar() else {
        return locations;
    };
    let Ok(Some(data)) = gvar.glyph_variation_data(gid) else {
        return locations;
    };
    for tuple in data.tuples() {
        let peak: Vec<f32> = (0..axis_count)
            .map(|i| {
                tuple
                    .peak()
                    .values()
                    .get(i)
                    .map_or(0.0, |v| v.get().to_f32())
            })
            .collect();
        if !locations.contains(&peak) {
            locations.push(peak);
        }
    }
    locations
}

/// Measure the Y extremes of every glyph in a font across its designspace.
fn font_metrics(font: &TestFont, metrics: &mut Metrics) -> Result<bool, FontspectorError> {
    let axis_count = font.font().axes().len();
    let outlines = font.font().outline_glyphs();
    let mut drew_any = false;
    for gid in font.all_glyphs() {
        let Some(glyph) = outlines.get(gid) else {
            continue;
        };
        for coords in glyph_locations(font, gid, axis_count) {
            let mut location = Location::new(axis_count);
            for (slot, value) in location.coords_mut().iter_mut().zip(&coords) {
                *slot = NormalizedCoord::from_f32(*value);
            }
            let mut pen = YBoundsPen::default();
            if glyph
                .draw(
                    DrawSettings::unhinted(Size::unscaled(), &location),
                    &mut pen,
                )
                .is_err()
            {
                continue;
            }
            let (Some(ymin), Some(ymax)) = (pen.ymin, pen.ymax) else {
                continue;
            };
            drew_any = true;
            let ymin = ymin.floor().clamp(i16::MIN as f32, i16::MAX as f32) as i16;
            let ymax = ymax.ceil().clamp(i16::MIN as f32, i16::MAX as f32) as i16;
            if ymin < metrics.ymin {
                metrics.ymin = ymin;
                metrics.lowest = Some(Extreme {
                    glyph: gid,
                    location: coords.clone(),
                });
            }
            if ymax > metrics.ymax {
                metrics.ymax = ymax;
                metrics.highest = Some(Extreme {
                    glyph: gid,
                    location: coords,
                });
            }
        }
    }
    Ok(drew_any)
}

fn family_metrics(fonts: &[TestFont]) -> Result<Metrics, FontspectorError> {
    let mut metrics = Metrics::default();
    for font in fonts {
        let drew_any = font_metrics(font, &mut metrics)?;
        if !drew_any {
            // No outlines we could draw (e.g. a bitmap-only font); fall back
            // to the head table's bounding box.
            let head = font.font().head()?;
            metrics.ymin = metrics.ymin.min(head.y_min());
            metrics.ymax = metrics.ymax.max(head.y_max());
        }
    }
    Ok(metrics)
}

#[check(
    id = "family/win_ascent_and_descent",
    rationale = "
        A font's winAscent and winDescent values should be greater than or equal to
        the highest and lowest points of any glyph outline in the family. If they are
        less than these values, clipping can occur on Windows platforms
        (https://github.com/RedHatBrand/Overpass/issues/33).

        For variable fonts, the outlines are measured across the designspace, at the
        peak of every gvar tuple, since the head table's yMin and yMax only describe
        the default location and glyphs can grow taller or deeper at other locations.

        If the font includes tall/deep writing systems such as Arabic or Devanagari,
        the winAscent and winDescent can be greater than the yMax and absolute yMin
        values to accommodate vowel marks.

        When the 'win' Metrics are significantly greater than the UPM, the linespacing
        can appear too loose. To counteract this, enabling the OS/2 fsSelection
        bit 7 (Use_Typo_Metrics), will force Windows to use the OS/2 'typo' values
        instead. This means the font developer can control the linespacing with
        the 'typo' values, whilst avoiding clipping by setting the 'win' values to
        values greater than the yMax and absolute yMin.
    ",
    implementation = "all",
    title = "Checking OS/2 usWinAscent & usWinDescent",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829"
)]
fn family_win_ascent_and_descent(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(c);
    let metrics = family_metrics(&fonts)?;
    let highest = metrics.describe(&fonts, &metrics.highest, "highest");
    let lowest = metrics.describe(&fonts, &metrics.lowest, "lowest");
    let mut problems = vec![];
    for font in fonts.iter() {
        let os2 = font.font().os2()?;
        if (os2.us_win_ascent() as i16) < metrics.ymax {
            let message = format!(
                "OS/2.usWinAscent value should be equal or greater than {}, but got {} instead.{}",
                metrics.ymax,
                os2.us_win_ascent(),
                highest
            );
            let mut status = Status::fail("ascent", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("usWinAscent".to_string()),
                actual: Some(json!(os2.us_win_ascent())),
                expected: Some(json!({ "min": metrics.ymax })),
                message: message.clone(),
            });
            problems.push(status);
        }
        if metrics.ymax > 0 && (os2.us_win_ascent() as i16) > 2 * metrics.ymax {
            let message = format!("OS/2.usWinAscent value {} is too large. It should be less than double the yMax. Current yMax value is {}.",
                os2.us_win_ascent(), metrics.ymax);
            let mut status = Status::fail("ascent", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("usWinAscent".to_string()),
                actual: Some(json!(os2.us_win_ascent())),
                expected: Some(json!({ "max": 2 * metrics.ymax })),
                message: message.clone(),
            });
            problems.push(status);
        }
        if (os2.us_win_descent() as i16) < metrics.ymin.abs() {
            let message = format!(
                "OS/2.usWinDescent value should be equal or greater than {}, but got {} instead.{}",
                metrics.ymin.abs(),
                os2.us_win_descent(),
                lowest
            );
            let mut status = Status::fail("descent", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("usWinDescent".to_string()),
                actual: Some(json!(os2.us_win_descent())),
                expected: Some(json!({ "min": metrics.ymin.abs() })),
                message: message.clone(),
            });
            problems.push(status);
        }
        if metrics.ymin < 0 && (os2.us_win_descent() as i16) > 2 * metrics.ymin.abs() {
            let message = format!("OS/2.usWinDescent value {} is too large. It should be less than double the yMin. Current absolute yMin value is {}.",
                os2.us_win_descent(), metrics.ymin.abs());
            let mut status = Status::fail("descent", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("usWinDescent".to_string()),
                actual: Some(json!(os2.us_win_descent())),
                expected: Some(json!({ "max": 2 * metrics.ymin.abs() })),
                message: message.clone(),
            });
            problems.push(status);
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::family_win_ascent_and_descent;
    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_results_contain, run_check_with_config, test_able,
        },
        StatusCode, TestableCollection, TestableType,
    };

    #[test]
    fn test_family_win_ascent_descent_fail() {
        let testables = vec![test_able("mada/Mada-Regular.ttf")];
        let collection = TestableCollection::from_testables(testables, None);
        let results = run_check_with_config(
            family_win_ascent_and_descent,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(&results, StatusCode::Fail, Some("ascent".to_string()));
    }

    #[test]
    fn test_variable_font_extremes_found_away_from_default() {
        // IBM Plex Sans VF passes the head-table check at the default
        // location, but Aringacute grows taller at wght=700.
        let testables = vec![test_able("ibmplexsans-vf/IBMPlexSansVar-Roman.ttf")];
        let collection = TestableCollection::from_testables(testables, None);
        let results = run_check_with_config(
            family_win_ascent_and_descent,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(&results, StatusCode::Fail, Some("ascent".to_string()));
        assert_messages_contain(
            &results,
            "should be equal or greater than 1150, but got 1025 instead. The highest point is in glyph 'Aringacute' at wght=700.",
        );
    }
}

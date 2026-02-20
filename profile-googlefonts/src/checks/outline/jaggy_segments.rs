use crate::checks::outline::name_and_bezglyph;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use itertools::Itertools;
use kurbo::{ParamCurve, ParamCurveDeriv, PathSeg, Point, Vec2};
use serde_json::json;

fn tangent_at_time(p: &PathSeg, t: f64) -> Vec2 {
    match p {
        PathSeg::Line(line) => line.deriv().eval(t),
        PathSeg::Quad(quad_bez) => quad_bez.deriv().eval(t),
        PathSeg::Cubic(cubic_bez) => cubic_bez.deriv().eval(t),
    }
    .to_vec2()
}

fn get_segment_points(seg: &PathSeg) -> (Point, Point) {
    match seg {
        PathSeg::Line(line) => (line.p0, line.p1),
        PathSeg::Quad(quad) => (quad.p0, quad.p2),
        PathSeg::Cubic(cubic) => (cubic.p0, cubic.p3),
    }
}

const JAG_ANGLE: f64 = 0.25; // Radians

struct JaggyWarning {
    glyph_name: String,
    glyph_id: u32,
    prev_seg: PathSeg,
    cur_seg: PathSeg,
    jag_angle_degrees: f64,
}

impl std::fmt::Display for JaggyWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?}/{:?} = {} degrees",
            self.glyph_name, self.prev_seg, self.cur_seg, self.jag_angle_degrees
        )
    }
}

#[check(
    id = "outline_jaggy_segments",
    rationale = "
        
        This check heuristically detects outline segments which form a particularly
        small angle, indicative of an outline error. This may cause false positives
        in cases such as extreme ink traps, so should be regarded as advisory and
        backed up by manual inspection.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3064",
    title = "Do outlines contain any jaggy segments?"
)]
fn jaggy_segments(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        f.is_variable_font(),
        "variable-font",
        "This check produces too many false positives with variable fonts."
    );
    let mut problems = vec![];
    let mut all_warnings: Vec<JaggyWarning> = vec![];

    for (glyph_id, (name, result)) in name_and_bezglyph(&f).enumerate() {
        let pen = result?;
        for path in pen.iter() {
            let segs = path.segments().collect::<Vec<_>>();
            for (prev, cur) in segs.iter().circular_tuple_windows() {
                let in_vector = tangent_at_time(prev, 1.0) * -1.0;
                let out_vector = tangent_at_time(cur, 0.0);
                if in_vector.length_squared() * out_vector.length_squared() == 0.0 {
                    continue;
                }
                let angle = in_vector.dot(out_vector) / (in_vector.length() * out_vector.length());
                if !(-1.0..=1.0).contains(&angle) {
                    continue;
                }
                let jag_angle = angle.acos();
                if jag_angle.abs() > JAG_ANGLE || jag_angle == 0.0 {
                    continue;
                }
                all_warnings.push(JaggyWarning {
                    glyph_name: name.to_string(),
                    glyph_id: glyph_id as u32,
                    prev_seg: *prev,
                    cur_seg: *cur,
                    jag_angle_degrees: jag_angle.to_degrees(),
                });
            }
        }
    }
    if !all_warnings.is_empty() {
        let mut status = Status::warn(
            "found-jaggy-segments",
            &format!(
                "The following glyphs have jaggy segments:\n\n{}",
                bullet_list(context, all_warnings.iter().map(|w| w.to_string()))
            ),
        );
        for warning in all_warnings {
            let (_, cur_start_point) = get_segment_points(&warning.cur_seg);
            let (prev_start_point, prev_end_point) = get_segment_points(&warning.prev_seg);
            let (_, cur_end_point) = get_segment_points(&warning.cur_seg);

            status.add_metadata(Metadata::GlyphProblem {
                glyph_name: warning.glyph_name,
                glyph_id: warning.glyph_id,
                position: Some((cur_start_point.x as f32, cur_start_point.y as f32)),
                message: "Jaggy segment found".to_string(),
                actual: Some(json!({
                    "prev_seg_start": [prev_start_point.x, prev_start_point.y],
                    "prev_seg_end": [prev_end_point.x, prev_end_point.y],
                    "cur_seg_start": [cur_start_point.x, cur_start_point.y],
                    "cur_seg_end": [cur_end_point.x, cur_end_point.y],
                    "jag_angle_degrees": warning.jag_angle_degrees,
                })),
                expected: None,
                userspace_location: None,
            });
        }
        problems.push(status);
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_messages_contain, assert_pass, assert_results_contain, run_check, test_able,
    };

    use fontspector_checkapi::StatusCode;

    #[test]
    fn test_outline_jaggy_segments() {
        let testable = test_able("wonky_paths/WonkySourceSansPro-Regular.ttf");
        let results = run_check(super::jaggy_segments, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("found-jaggy-segments".to_string()),
        );
        assert_messages_contain(&results, "E (U+0045)");

        let testable = test_able("familysans/FamilySans-Regular.ttf");
        let results = run_check(super::jaggy_segments, testable);
        assert_pass(&results);

        let testable = test_able("source-sans-pro/OTF/SourceSansPro-LightItalic.otf");
        let results = run_check(super::jaggy_segments, testable);
        assert_pass(&results);

        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Roman.otf");
        let results = run_check(super::jaggy_segments, testable);
        assert_results_contain(
            &results,
            StatusCode::Skip,
            Some("variable-font".to_string()),
        );
    }
}

use super::name_and_bezglyph;
use crate::checks::outline::close_but_not_on;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use kurbo::{PathSeg, Point};
use serde_json::json;

fn get_segment_points(seg: &PathSeg) -> (Point, Point) {
    match seg {
        PathSeg::Line(line) => (line.p0, line.p1),
        PathSeg::Quad(quad) => (quad.p0, quad.p2),
        PathSeg::Cubic(cubic) => (cubic.p0, cubic.p3),
    }
}

struct SemiVerticalWarning {
    glyph_name: String,
    glyph_id: u32,
    segment: PathSeg,
    actual_angle: f64,
    expected_angle: f64,
}

impl std::fmt::Display for SemiVerticalWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?} (angle: {:.2} degrees, expected: {:.2} degrees)",
            self.glyph_name, self.segment, self.actual_angle, self.expected_angle
        )
    }
}

#[check(
    id = "outline_semi_vertical",
    rationale = "
        
        This check detects line segments which are nearly, but not quite, exactly
        horizontal or vertical. Sometimes such lines are created by design, but often
        they are indicative of a design error.

        This check is disabled for italic styles, which often contain nearly-upright
        lines.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3088",
    title = "Do outlines contain any semi-vertical or semi-horizontal lines?"
)]
fn semi_vertical(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut all_warnings: Vec<SemiVerticalWarning> = vec![];
    skip!(
        f.is_variable_font(),
        "variable-font",
        "This check produces too many false positives with variable fonts."
    );
    skip!(
        f.is_italic()?,
        "italic",
        "This check produces too many false positives with italic fonts."
    );

    for (glyph_id, (name, result)) in name_and_bezglyph(&f).enumerate() {
        let pen = result?;
        for path in pen.iter() {
            for seg in path.segments() {
                if let kurbo::PathSeg::Line(line) = seg {
                    let angle = (line.p1 - line.p0).angle().to_degrees();
                    for y_expected in [-180.0, -90.0, 0.0, 90.0, 180.0] {
                        if close_but_not_on(angle, y_expected, 0.5) {
                            all_warnings.push(SemiVerticalWarning {
                                glyph_name: name.to_string(),
                                glyph_id: glyph_id as u32,
                                segment: seg,
                                actual_angle: angle,
                                expected_angle: y_expected,
                            });
                        }
                    }
                }
            }
        }
    }
    if !all_warnings.is_empty() {
        let mut status = Status::warn(
            "found-semi-vertical",
            &format!(
                "The following glyphs have semi-vertical/semi-horizontal lines:\n\n{}",
                bullet_list(context, all_warnings.iter().map(|w| w.to_string()))
            ),
        );
        for warning in all_warnings {
            let (start_point, _) = get_segment_points(&warning.segment);
            status.add_metadata(Metadata::GlyphProblem {
                glyph_name: warning.glyph_name,
                glyph_id: warning.glyph_id,
                position: Some((start_point.x as f32, start_point.y as f32)),
                message: "Semi-vertical/horizontal line found".to_string(),
                actual: Some(json!({
                    "segment": format!("{:?}", warning.segment),
                    "actual_angle": warning.actual_angle,
                })),
                expected: Some(json!({
                    "expected_angle": warning.expected_angle,
                })),
                userspace_location: None,
            });
        }
        return Ok(Box::new(vec![status].into_iter()));
    }
    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_messages_contain, assert_results_contain, run_check, test_able,
    };

    use fontspector_checkapi::StatusCode;

    #[test]
    fn test_outline_semi_vertical() {
        let testable = test_able("wonky_paths/WonkySourceSansPro-Regular.ttf");
        let results = run_check(super::semi_vertical, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("found-semi-vertical".to_string()),
        );
        assert_messages_contain(&results, "B (U+0042)");

        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Roman.otf");
        let results = run_check(super::semi_vertical, testable);
        assert_results_contain(
            &results,
            StatusCode::Skip,
            Some("variable-font".to_string()),
        );

        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Italic.otf");
        let results = run_check(super::semi_vertical, testable);
        assert_results_contain(&results, StatusCode::Skip, Some("italic".to_string()));
    }
}

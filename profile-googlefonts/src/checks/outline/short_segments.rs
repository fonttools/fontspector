use crate::checks::outline::name_and_bezglyph;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use kurbo::{ParamCurveArclen, PathSeg, Point, Shape};
use serde_json::json;

const SHORT_PATH_ABSOLUTE_EPSILON: f64 = 3.0;
const SHORT_PATH_RELATIVE_EPSILON: f64 = 0.006;

fn get_segment_points(seg: &PathSeg) -> (Point, Point) {
    match seg {
        PathSeg::Line(line) => (line.p0, line.p1),
        PathSeg::Quad(quad) => (quad.p0, quad.p2),
        PathSeg::Cubic(cubic) => (cubic.p0, cubic.p3),
    }
}

fn segment_is_short(pathseg: &PathSeg, total_length: f64, prev_was_line: bool) -> bool {
    let len = pathseg.arclen(0.01);
    // An *very* short segment is likely to be a mistake
    if len <= 1.0e-9 {
        return true;
    }
    let short_seg =
        len < SHORT_PATH_ABSOLUTE_EPSILON || len < SHORT_PATH_RELATIVE_EPSILON * total_length;
    let current_is_curve = matches!(pathseg, PathSeg::Cubic(_) | PathSeg::Quad(_));

    short_seg && (prev_was_line || current_is_curve)
}

struct ShortSegmentWarning {
    glyph_name: String,
    glyph_id: u32,
    segment: PathSeg,
    segment_length: f64,
    total_outline_length: f64,
}

impl std::fmt::Display for ShortSegmentWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} contains a short segment {:?} (length: {:.2}, total outline: {:.2})",
            self.glyph_name, self.segment, self.segment_length, self.total_outline_length
        )
    }
}

#[check(
    id = "outline_short_segments",
    rationale = "
        
        This check looks for outline segments which seem particularly short (less
        than 0.6% of the overall path length).

        This check is not run for variable fonts, as they may legitimately have
        short segments. As this check is liable to generate significant numbers
        of false positives, it will pass if there are more than
        100 reported short segments.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3088",
    title = "Are any segments inordinately short?"
)]
fn short_segments(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut all_warnings: Vec<ShortSegmentWarning> = vec![];
    skip!(
        f.is_variable_font(),
        "variable-font",
        "This check produces too many false positives with variable fonts."
    );
    for (glyph_id, (name, result)) in name_and_bezglyph(&f).enumerate() {
        let pen = result?;
        for path in pen.iter() {
            let outline_length = path.perimeter(0.01);
            let segments = path.segments().collect::<Vec<_>>();
            if segments.is_empty() {
                continue;
            }
            #[allow(clippy::unwrap_used)] // We just checked it has a segment
            let mut prev_was_line = matches!(segments.last().unwrap(), kurbo::PathSeg::Line(_));
            for seg in segments.iter() {
                if segment_is_short(seg, outline_length, prev_was_line) {
                    all_warnings.push(ShortSegmentWarning {
                        glyph_name: name.to_string(),
                        glyph_id: glyph_id as u32,
                        segment: *seg,
                        segment_length: seg.arclen(0.01),
                        total_outline_length: outline_length,
                    });
                }
                prev_was_line = matches!(seg, kurbo::PathSeg::Line(_));
            }
            if all_warnings.len() > 100 {
                return Ok(Status::just_one_pass());
            }
        }
    }
    if !all_warnings.is_empty() {
        let mut status = Status::warn(
            "found-short-segments",
            &format!(
                "The following glyphs have short segments:\n\n{}",
                bullet_list(context, all_warnings.iter().map(|w| w.to_string()))
            ),
        );
        for warning in all_warnings {
            let (start_point, _) = get_segment_points(&warning.segment);
            status.add_metadata(Metadata::GlyphProblem {
                glyph_name: warning.glyph_name,
                glyph_id: warning.glyph_id,
                position: Some((start_point.x as f32, start_point.y as f32)),
                message: "Short segment found".to_string(),
                actual: Some(json!({
                    "segment": format!("{:?}", warning.segment), // Serialize PathSeg as string for now
                    "segment_length": warning.segment_length,
                    "total_outline_length": warning.total_outline_length,
                })),
                expected: None,
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
    fn test_outline_short_segments() {
        let testable = test_able("wonky_paths/WonkySourceSansPro-Regular.ttf");
        let results = run_check(super::short_segments, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("found-short-segments".to_string()),
        );
        assert_messages_contain(
            &results,
            "D (U+0044) contains a short segment Line(Line { p0: (180.0, 68.0)",
        );

        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Roman.otf");
        let results = run_check(super::short_segments, testable);
        assert_results_contain(
            &results,
            StatusCode::Skip,
            Some("variable-font".to_string()),
        );
    }
}

use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use itertools::Itertools;
use serde_json::json;

use crate::checks::outline::name_and_bezglyph;

const COLINEAR_EPSILON: f64 = 0.1; // Radians

struct ColinearWarning {
    glyph_name: String,
    glyph_id: u32,
    p0: kurbo::Point,
    p1: kurbo::Point,
    p2: kurbo::Point,
}

impl std::fmt::Display for ColinearWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: from {:?} to {:?} is colinear with segment from {:?} to {:?}",
            self.glyph_name, self.p0, self.p1, self.p1, self.p2
        )
    }
}

#[check(
    id = "outline_colinear_vectors",
    rationale = "
        
        This check looks for consecutive line segments which have the same angle. This
        normally happens if an outline point has been added by accident.

        This check is not run for variable fonts, as they may legitimately have
        colinear vectors.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3088",
    title = "Do any segments have colinear vectors?"
)]
fn colinear_vectors(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut all_warnings: Vec<ColinearWarning> = vec![];
    skip!(
        f.is_variable_font(),
        "variable-font",
        "This check produces too many false positives with variable fonts."
    );
    for (glyph_id, (name, result)) in name_and_bezglyph(&f).enumerate() {
        let pen = result?;
        for contour in pen.iter() {
            let segs = contour.segments().collect::<Vec<_>>();
            for (prev, next) in segs.iter().circular_tuple_windows() {
                if let (kurbo::PathSeg::Line(prev), kurbo::PathSeg::Line(next)) = (prev, next) {
                    let prev_angle = (prev.p1 - prev.p0).angle();
                    let next_angle = (next.p1 - next.p0).angle();
                    if (prev_angle - next_angle).abs() < COLINEAR_EPSILON {
                        all_warnings.push(ColinearWarning {
                            glyph_name: name.to_string(),
                            glyph_id: glyph_id as u32,
                            p0: prev.p0,
                            p1: prev.p1,
                            p2: next.p1,
                        });
                    }
                }
            }
            if all_warnings.len() > 100 {
                return Ok(Status::just_one_pass());
            }
        }
    }
    if !all_warnings.is_empty() {
        let mut status = Status::warn(
            "found-colinear-vectors",
            &format!(
                "The following glyphs have colinear vectors:\n\n{}",
                bullet_list(context, all_warnings.iter().map(|w| w.to_string()))
            ),
        );
        for warning in all_warnings {
            status.add_metadata(Metadata::GlyphProblem {
                glyph_name: warning.glyph_name,
                glyph_id: warning.glyph_id,
                userspace_location: None,
                position: Some((warning.p1.x as f32, warning.p1.y as f32)),
                message: "Colinear vector".to_string(),
                actual: Some(json!({
                    "p0": [warning.p0.x, warning.p0.y],
                    "p1": [warning.p1.x, warning.p1.y],
                    "p2": [warning.p2.x, warning.p2.y],
                })),
                expected: None,
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
    fn test_outline_colinear_vectors() {
        let testable = test_able("wonky_paths/WonkySourceSansPro-Regular.otf");
        let results = run_check(super::colinear_vectors, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("found-colinear-vectors".to_string()),
        );
        assert_messages_contain(&results, "C (U+0043)");
        assert_messages_contain(&results, "E (U+0045)");

        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Roman.otf");
        let results = run_check(super::colinear_vectors, testable);
        assert_results_contain(
            &results,
            StatusCode::Skip,
            Some("variable-font".to_string()),
        );
    }
}

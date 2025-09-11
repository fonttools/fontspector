use crate::checks::outline::name_and_bezglyph;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use itertools::Itertools;
use kurbo::{ParamCurve, ParamCurveDeriv, PathSeg, Vec2};

fn tangent_at_time(p: &PathSeg, t: f64) -> Vec2 {
    match p {
        PathSeg::Line(line) => line.deriv().eval(t),
        PathSeg::Quad(quad_bez) => quad_bez.deriv().eval(t),
        PathSeg::Cubic(cubic_bez) => cubic_bez.deriv().eval(t),
    }
    .to_vec2()
}

const JAG_ANGLE: f64 = 0.25; // Radians

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
    let mut all_warnings = vec![];

    for (name, result) in name_and_bezglyph(&f) {
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
                let jag_angle = angle.acos(); // Why did I do this?
                if jag_angle.abs() > JAG_ANGLE || jag_angle == 0.0 {
                    continue;
                }
                all_warnings.push(format!(
                    "{}: {:?}/{:?} = {}",
                    name,
                    prev,
                    cur,
                    jag_angle.to_degrees()
                ));
            }
        }
    }
    if !all_warnings.is_empty() {
        problems.push(Status::warn(
            "found-jaggy-segments",
            &format!(
                "The following glyphs have jaggy segments:\n\n{}",
                bullet_list(context, all_warnings)
            ),
        ));
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

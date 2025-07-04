use super::name_and_bezglyph;
use crate::checks::outline::close_but_not_on;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

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
    let mut all_warnings = vec![];
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

    for (name, result) in name_and_bezglyph(&f) {
        let pen = result?;
        for path in pen.iter() {
            for seg in path.segments() {
                if let kurbo::PathSeg::Line(line) = seg {
                    let angle = (line.p1 - line.p0).angle().to_degrees();
                    for y_expected in [-180.0, -90.0, 0.0, 90.0, 180.0] {
                        if close_but_not_on(angle, y_expected, 0.5) {
                            all_warnings.push(format!("{name}: {seg:?}"));
                        }
                    }
                }
            }
        }
    }
    Ok(if !all_warnings.is_empty() {
        Status::just_one_warn(
            "found-semi-vertical",
            &format!(
                "The following glyphs have semi-vertical/semi-horizontal lines:\n\n{}",
                bullet_list(context, all_warnings)
            ),
        )
    } else {
        Status::just_one_pass()
    })
}

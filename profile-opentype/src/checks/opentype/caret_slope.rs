use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "opentype/caret_slope",
    title = "Check hhea.caretSlopeRise and hhea.caretSlopeRun",
    proposal = "https://github.com/fonttools/fontbakery/issues/3670",
    rationale = r#"
        Checks whether hhea.caretSlopeRise and hhea.caretSlopeRun
        match with post.italicAngle.

        For Upright fonts, you can set hhea.caretSlopeRise to 1
        and hhea.caretSlopeRun to 0.

        For Italic fonts, you can set hhea.caretSlopeRise to head.unitsPerEm
        and calculate hhea.caretSlopeRun like this:
        round(math.tan(
          math.radians(-1 * font["post"].italicAngle)) * font["head"].unitsPerEm)

        This check allows for a 0.1° rounding difference between the Italic angle
        as calculated by the caret slope and post.italicAngle
    "#
)]
fn caret_slope(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let post_italic_angle = f.font().post()?.italic_angle().to_f32();
    let upem = f.font().head()?.units_per_em();
    let run = f.font().hhea()?.caret_slope_run();
    let rise = f.font().hhea()?.caret_slope_rise();
    if rise == 0 {
        return Ok(Status::just_one_fail(
            "zero-rise",
            "caretSlopeRise must not be zero. Set it to 1 for upright fonts.",
        ));
    }
    let hhea_angle = (-run as f32 / rise as f32).atan().to_degrees();
    let expected_run = (-post_italic_angle.to_radians().tan() * upem as f32).round() as i16;
    let expected_rise = if expected_run == 0 { 1 } else { upem };
    if (post_italic_angle - hhea_angle).abs() > 0.1 {
        return Ok(Status::just_one_warn(
            "mismatch",
            &format!(
                "hhea.caretSlopeRise and hhea.caretSlopeRun do not match with post.italicAngle.
                Got caretSlopeRise: {}, caretSlopeRun: {}, expected caretSlopeRise: {}, caretSlopeRun: {}",
                rise, run, expected_rise, expected_run
            ),
        ));
    }
    Ok(Status::just_one_pass())
}

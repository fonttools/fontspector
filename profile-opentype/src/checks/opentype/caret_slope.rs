use fontations::{skrifa::raw::TableProvider, types::Fixed, write::from_obj::ToOwnedTable};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

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
    "#,
    hotfix = fix_caret_slope,
)]
fn caret_slope(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let post_italic_angle = f.font().post()?.italic_angle().to_f32();
    let upem = f.font().head()?.units_per_em();
    let run = f.font().hhea()?.caret_slope_run();
    let rise = f.font().hhea()?.caret_slope_rise();
    let mut problems = vec![];
    if rise == 0 {
        let msg = "caretSlopeRise must not be zero. Set it to 1 for upright fonts.";
        let mut status = Status::fail("zero-rise", msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "hhea".to_string(),
            field_name: Some("caretSlopeRise".to_string()),
            actual: Some(json!(0)),
            expected: Some(json!(1)),
            message: msg.to_string(),
        });
        problems.push(status);
        return return_result(problems);
    }
    let hhea_angle = (-run as f32 / rise as f32).atan().to_degrees();
    let expected_run = (-post_italic_angle.to_radians().tan() * upem as f32).round() as i16;
    let expected_rise = if expected_run == 0 { 1 } else { upem };
    if (post_italic_angle - hhea_angle).abs() > 0.1 {
        let msg = format!(
            "hhea.caretSlopeRise and hhea.caretSlopeRun do not match with post.italicAngle.\n\
                Got caretSlopeRise: {rise}, caretSlopeRun: {run}, expected caretSlopeRise: {expected_rise}, caretSlopeRun: {expected_run}"
        );
        let mut status = Status::warn("mismatch", &msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "hhea".to_string(),
            field_name: Some("caretSlopeRise/caretSlopeRun".to_string()),
            actual: Some(json!({
                "caretSlopeRise": rise,
                "caretSlopeRun": run,
                "calculated_angle": hhea_angle
            })),
            expected: Some(json!({
                "caretSlopeRise": expected_rise,
                "caretSlopeRun": expected_run,
                "expected_angle": post_italic_angle
            })),
            message: msg,
        });
        problems.push(status);
    }
    return_result(problems)
}

// fn fix_post_italic_angle(t: &mut Testable) -> FixFnResult {
//     let f = fixfont!(t);
//     let Some(style) = f.style() else {
//         return Ok(false);
//     };
//     let mut post: fontations::write::tables::post::Post = f
//         .font()
//         .post()
//         .map_err(|e| format!("Couldn't get post table: {}", e))?
//         .to_owned_table();
//     let hhea = f
//         .font()
//         .hhea()
//         .map_err(|e| format!("Couldn't get hhea table: {}", e))?;
//     if !style.contains("Italic") {
//         post.italic_angle = 0.into();
//     } else {
//         let run = hhea.caret_slope_run();
//         let rise = hhea.caret_slope_rise();
//         let angle = (-run as f64 / rise as f64).atan().to_degrees();
//         post.italic_angle = Fixed::from_f64(angle);
//     }
//     t.set(f.rebuild_with_new_tables(&[post])?);
//     Ok(true)
// }

fn fix_caret_slope(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    let mut hhea: fontations::write::tables::hhea::Hhea = f.font().hhea()?.to_owned_table();
    let post = f.font().post()?;
    if post.italic_angle() == Fixed::ZERO {
        println!("Skipping fix_caret_slope for non-italic font");
        return Ok(false);
    }
    let upem = f.font().head()?.units_per_em();
    hhea.caret_slope_rise = upem as i16;
    hhea.caret_slope_run =
        (-post.italic_angle().to_f32().to_radians().tan() * upem as f32).round() as i16;
    t.set(f.rebuild_with_new_table(&hhea)?);
    Ok(true)
}

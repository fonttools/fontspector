use fontations::skrifa::{
    metrics::GlyphMetrics,
    prelude::{LocationRef, Size},
    MetadataProvider,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

use crate::checks::iso15008::find_stem_width;

#[check(
    id = "iso15008/interline_spacing",
    rationale = "
        
        According to ISO 15008, fonts used for in-car displays
        should not be too narrow or too wide.

        To ensure legibility of this font on in-car information systems,
        it is recommended that the vertical metrics be set to a minimum
        at least one stem width between the bottom of the descender
        and the top of the ascender.
    
        (Note that passing this check does not guarantee compliance with ISO-15008.)

    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/1832 and https://github.com/fonttools/fontbakery/issues/3254",
    title = "Check if spacing between lines is adequate for display use"
)]
fn interline_spacing(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let Some(h_id) = f.font().charmap().map('h') else {
        return Ok(Status::just_one_fail("no-h", "Could not find h glyph"));
    };
    let Some(h_bounds) =
        GlyphMetrics::new(&f.font(), Size::unscaled(), LocationRef::new(&[])).bounds(h_id)
    else {
        return Ok(Status::just_one_fail(
            "no-h-bounds",
            "Could not find h bounds",
        ));
    };
    let Some(g_id) = f.font().charmap().map('g') else {
        return Ok(Status::just_one_fail("no-g", "Could not find g glyph"));
    };
    let Some(g_bounds) =
        GlyphMetrics::new(&f.font(), Size::unscaled(), LocationRef::new(&[])).bounds(g_id)
    else {
        return Ok(Status::just_one_fail(
            "no-g-bounds",
            "Could not find g bounds",
        ));
    };
    let vmetrics = f.vertical_metrics()?;
    let linegap = g_bounds.y_min - vmetrics.os2_typo_descender as f32
        + vmetrics.os2_typo_linegap as f32
        + (vmetrics.os2_typo_ascender as f32 - h_bounds.y_max);
    let Some(width) = find_stem_width(&f) else {
        return Ok(Status::just_one_fail(
            "no-stem-width",
            "Could not determine stem width",
        ));
    };
    if (linegap as f64) < width {
        return Ok(Status::just_one_fail(
            "bad-interline-spacing",
            &format!("The interline space {linegap} should be more than the stem width {width}"),
        ));
    }
    Ok(Status::just_one_pass())
}

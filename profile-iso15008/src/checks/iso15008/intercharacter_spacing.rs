use fontations::skrifa::{
    metrics::GlyphMetrics,
    prelude::{LocationRef, Size},
    raw::TableProvider,
    MetadataProvider,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

use crate::checks::iso15008::{find_stem_width, x_height_intersections};

use super::pair_kerning;

#[check(
    id = "iso15008/intercharacter_spacing",
    rationale = "
        
        According to ISO 15008, fonts used for in-car displays should not
        be too narrow or too wide.

        To ensure legibility of this font on in-car information systems,
        it is recommended that the spacing falls within the following values:

        * space between vertical strokes (e.g. \"ll\") should be 150%-240%
          of the stem width.

        * space between diagonals and verticals (e.g. \"vl\") should be
          at least 85% of the stem width.

        * diagonal characters should not touch (e.g. \"vv\").
    
        (Note that passing this check does not guarantee compliance with ISO-15008.)

    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/1832 and https://github.com/fonttools/fontbakery/issues/3252",
    title = "Check if spacing between characters is adequate for display use"
)]
fn intercharacter_spacing(t: &Testable, _context: &Context) -> CheckFnResult {
    let contents = &t.contents;
    let f = testfont!(t);
    let mut problems = vec![];
    let Some(width) = find_stem_width(&f) else {
        return Ok(Status::just_one_fail(
            "no-stem-width",
            "Could not determine stem width",
        ));
    };

    // Because an l can have a curly tail, we don't want the *glyph* sidebearings;
    // we want the sidebearings measured using a line at Y=x-height.
    let Some(l_id) = f.font().charmap().map('l') else {
        return Ok(Status::just_one_fail("no-l", "Could not find l glyph"));
    };
    let Some(l_intersections) = x_height_intersections(&f, l_id) else {
        return Ok(Status::just_one_fail(
            "no-l-intersections",
            "Could not find l intersections",
        ));
    };
    if l_intersections.len() < 2 {
        return Ok(Status::just_one_fail(
            "no-l-intersections",
            "Could not find l intersections",
        ));
    }
    #[allow(clippy::indexing_slicing)]
    // We know l_intersections has at least two elements, so this is safe.
    let l_lsb = l_intersections[0];
    let l_advance = f.font().hmtx()?.advance(l_id).unwrap_or(0) as f64;
    #[allow(clippy::indexing_slicing)]
    let l_rsb = l_advance - (l_intersections[1] - l_intersections[0]);
    let Some(kern) = pair_kerning(contents, 'l', 'l') else {
        return Ok(Status::just_one_fail(
            "no-kern",
            "Could not find kerning between l and l",
        ));
    };
    let l_l = l_rsb + kern as f64 + l_lsb;
    if !(1.5..=2.4).contains(&(l_l / width)) {
        problems.push(Status::fail(
            "bad-vertical-vertical-spacing",
            &format!(
                "The space between vertical strokes ({}) does not conform to the expected range of {}-{}",
                l_l,
                width * 1.5,
                width * 2.4
            ),
        ));
    }

    // For v, however, a simple LSB/RSB is adequate.
    let Some(v_id) = f.font().charmap().map('v') else {
        return Ok(Status::just_one_fail("no-v", "Could not find v glyph"));
    };
    let Some(bounds) =
        GlyphMetrics::new(&f.font(), Size::unscaled(), LocationRef::new(&[])).bounds(v_id)
    else {
        return Ok(Status::just_one_fail(
            "no-bounds",
            "There was no bounds for the 'v' glyph in the font, so the proportions could not be tested",
        ));
    };
    let v_advance = f.font().hmtx()?.advance(v_id).unwrap_or(0) as f64;
    let v_lsb = bounds.x_min as f64;
    let v_rsb = v_advance - (bounds.x_max as f64 - bounds.x_min as f64);
    let Some(kern) = pair_kerning(contents, 'l', 'v') else {
        return Ok(Status::just_one_fail(
            "no-kern",
            "Could not find kerning between l and v",
        ));
    };
    let v_l = l_rsb + kern as f64 + v_lsb;
    if v_l <= 0.85 * width {
        problems.push(Status::fail(
            "bad-vertical-diagonal-spacing",
            &format!(
                "The space between vertical and diagonal strokes ({}) does not conform to the expected range of {}",
                v_l,
                width * 0.85
            ),
        ));
    }
    let Some(kern) = pair_kerning(contents, 'v', 'v') else {
        return Ok(Status::just_one_fail(
            "no-kern",
            "Could not find kerning between v and v",
        ));
    };
    if v_rsb + kern as f64 + v_lsb <= 0.0 {
        problems.push(Status::fail(
            "bad-diagonal-diagonal-spacing",
            "Diagonal strokes (vv) were touching",
        ));
    }
    return_result(problems)
}

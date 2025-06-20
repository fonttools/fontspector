use fontations::skrifa::{
    prelude::{LocationRef, Size},
    raw::TableProvider,
    MetadataProvider,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

use crate::checks::iso15008::{pair_kerning, x_height_intersections};

#[check(
    id = "iso15008/interword_spacing",
    rationale = "
        
        According to ISO 15008, fonts used for in-car displays
        should not be too narrow or too wide.

        To ensure legibility of this font on in-car information systems,
        it is recommended that the space character should have advance width
        between 250% and 300% of the space between the letters l and m.
    
        (Note that passing this check does not guarantee compliance with ISO-15008.)

    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/1832 and https://github.com/fonttools/fontbakery/issues/3253",
    title = "Check if spacing between words is adequate for display use"
)]
fn interword_spacing(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
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
            "There was no 'l' glyph in the font, so the spacing could not be tested",
        ));
    }

    let l_advance = f.font().hmtx()?.advance(l_id).unwrap_or(0);
    #[allow(clippy::indexing_slicing)] // We checked the length
    let l_rsb = l_advance as f64 - l_intersections[1];

    let Some(m_id) = f.font().charmap().map('m') else {
        return Ok(Status::just_one_fail("no-m", "Could not find m glyph"));
    };
    let m_advance = f.font().hmtx()?.advance(m_id).unwrap_or(0);
    let Some(m_bounds) = f
        .font()
        .glyph_metrics(Size::unscaled(), LocationRef::new(&[]))
        .bounds(m_id)
    else {
        return Ok(Status::just_one_fail(
            "no-m-bounds",
            "Could not find m bounds",
        ));
    };

    let m_lsb = m_bounds.x_min as f64;
    let m_rsb = m_advance as f64 - (m_lsb + m_bounds.x_max as f64 - m_bounds.x_min as f64);

    let Some(n_id) = f.font().charmap().map('n') else {
        return Ok(Status::just_one_fail("no-n", "Could not find n glyph"));
    };
    let n_lsb = f.font().hmtx()?.side_bearing(n_id).unwrap_or(0) as f64;
    let Some(kern) = pair_kerning(&t.contents, 'l', 'm') else {
        return Ok(Status::just_one_fail(
            "no-kern",
            "Could not find kern between l and m",
        ));
    };
    let l_m = l_rsb + kern as f64 + m_lsb;
    let Some(space_id) = f.font().charmap().map(' ') else {
        return Ok(Status::just_one_fail(
            "no-space",
            "Could not find space glyph",
        ));
    };
    let space_width = f.font().hmtx()?.advance(space_id).unwrap_or(0) as f64;
    // Add spacing caused by normal sidebearings
    let space_width = space_width + m_rsb + n_lsb;
    if !(2.50..=3.0).contains(&(space_width / l_m)) {
        return Ok(Status::just_one_fail(
            "bad-interword-spacing",
            &format!(
                "The interword space ({}) was outside the recommended range ({}-{})",
                space_width,
                l_m * 2.50,
                l_m * 3.0
            ),
        ));
    }
    Ok(Status::just_one_pass())
}

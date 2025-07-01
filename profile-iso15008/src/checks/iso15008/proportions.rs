use fontations::skrifa::{
    metrics::GlyphMetrics,
    prelude::{LocationRef, Size},
    MetadataProvider,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "iso15008/proportions",
    rationale = "
        
        According to ISO 15008, fonts used for in-car displays should not be
        too narrow or too wide.

        To ensure legibility of this font on in-car information systems,
        it is recommended that the ratio of H width to H height
        is between 0.65 and 0.80.
    
        (Note that passing this check does not guarantee compliance with ISO-15008.)

    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/1832 and https://github.com/fonttools/fontbakery/issues/3250",
    title = "Check if 0.65 => (H width / H height) => 0.80"
)]
fn proportions(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let Some(h_glyph) = f.font().charmap().map('H') else {
        return Ok(Status::just_one_fail(
            "no-glyph",
            "There was no 'H' glyph in the font, so the proportions could not be tested",
        ));
    };
    let Some(bounds) =
        GlyphMetrics::new(&f.font(), Size::unscaled(), LocationRef::new(&[])).bounds(h_glyph)
    else {
        return Ok(Status::just_one_fail(
            "no-bounds",
            "There was no bounds for the 'H' glyph in the font, so the proportions could not be tested",
        ));
    };
    let proportion = (bounds.x_max - bounds.x_min) / (bounds.y_max - bounds.y_min);
    if !(0.65..=0.80).contains(&proportion) {
        return Ok(Status::just_one_fail(
            "invalid-proportion",
            &format!(
                "The proportion of H width to H height ({proportion}) does not conform to the expected range of 0.65-0.80"
            ),
        ));
    }
    Ok(Status::just_one_pass())
}

use super::find_stem_width;
use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "iso15008/stem_width",
    rationale = "
        
        According to ISO 15008, fonts used for in-car displays should
        not be too light or too bold.

        To ensure legibility of this font on in-car information systems,
        it is recommended that the ratio of stem width to ascender height
        is between 0.10 and 0.20.
    
        (Note that passing this check does not guarantee compliance with ISO-15008.)

    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/1832 and https://github.com/fonttools/fontbakery/issues/3251",
    title = "Check if 0.10 <= (stem width / ascender) <= 0.82"
)]
fn stem_width(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let Some(width) = find_stem_width(&f) else {
        return Ok(Status::just_one_fail(
            "no-stem-width",
            "Could not determine stem width",
        ));
    };
    let ascender = f.font().hhea()?.ascender().to_i16();
    let proportion = width as f32 / ascender as f32;
    if !(0.10..=0.20).contains(&proportion) {
        return Ok(Status::just_one_fail(
            "invalid-proportion",
            &format!(
                "The proportion of stem width to ascender ({proportion}) does not conform to the expected range of 0.10-0.20"
            ),
        ));
    }
    Ok(Status::just_one_pass())
}

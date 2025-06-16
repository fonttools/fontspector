use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, skip, FileTypeConvert};
use hashbrown::{HashMap, HashSet};

#[check(
    id = "googlefonts/family/tnum_horizontal_metrics",
    rationale = "
        
        Tabular figures need to have the same metrics in all styles in order to allow
        tables to be set with proper typographic control, but to maintain the placement
        of decimals and numeric columns between rows.

        Here's a good explanation of this:
        https://www.typography.com/techniques/fonts-for-financials/#tabular-figs
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2278",
    title = "All tabular figures must have the same width across the RIBBI-family.",
    implementation = "all"
)]
fn tnum_horizontal_metrics(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(c);
    skip!(
        fonts.len() < 2,
        "no-siblings",
        "This check requires at least two sibling fonts to compare codepoint coverage."
    );
    let mut tnum_widths: HashMap<u16, HashSet<String>> = HashMap::new();
    for font in fonts {
        let hmtx = font.font().hmtx()?;
        for (tnum_glyph_id, tnum_glyph_name) in font
            .all_glyphs()
            .flat_map(|g| font.glyph_name_for_id(g).map(|name| (g, name)))
            .filter(|(_, s)| s.ends_with(".tnum"))
        {
            if let Some(width) = hmtx.advance(tnum_glyph_id) {
                tnum_widths
                    .entry(width)
                    .or_insert_with(HashSet::new)
                    .insert(tnum_glyph_name.to_string());
            }
        }
    }
    if tnum_widths.len() > 1 {
        #[allow(clippy::unwrap_used)] // We checked the length above
        let most_common = tnum_widths
            .iter()
            .max_by_key(|(_, glyphs)| glyphs.len())
            .unwrap();
        Ok(Status::just_one_fail(
            "inconsistent-widths",
            &format!(
                "The most common tabular glyph width is {}. But there are other tabular glyphs with different widths such as the following ones:\n\t{}",
                most_common.0,
                bullet_list(context,
                tnum_widths
                    .iter()
                    .filter(|(width, _)| *width != most_common.0)
                    .map(|(width, glyphs)| {
                        format!(
                            "Width: {} - Glyphs: {}",
                            width,
                            glyphs.iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(", ")
                        )
                    })
                )
            ),
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}

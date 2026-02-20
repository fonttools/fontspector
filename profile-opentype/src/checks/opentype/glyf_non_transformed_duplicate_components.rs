use fontations::skrifa::{
    raw::{
        tables::glyf::{Anchor, Glyph},
        TableProvider,
    },
    GlyphId,
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;
use std::collections::HashSet;

#[check(
    id = "opentype/glyf_non_transformed_duplicate_components",
    rationale = "
        There have been cases in which fonts had faulty double quote marks, with each
        of them containing two single quote marks as components with the same
        x, y coordinates which makes them visually look like single quote marks.

        This check ensures that glyphs do not contain duplicate components
        which have the same x,y coordinates.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/2709",
    title = "Check glyphs do not have duplicate components which have the same x,y coordinates."
)]
fn glyf_non_transformed_duplicate_components(t: &Testable, _context: &Context) -> CheckFnResult {
    let ttf = testfont!(t);
    let font = ttf.font();
    skip!(!ttf.has_table(b"glyf"), "no-glyf", "No glyf table");
    let glyf = font.glyf()?;
    let loca = font.loca(None)?;
    let mut problems = vec![];
    for gid in 0..font.maxp()?.num_glyphs() {
        let gid = GlyphId::new(gid.into());
        if let Some(Glyph::Composite(glyph)) = loca.get_glyf(gid, &glyf)? {
            let mut components = HashSet::new();
            for component in glyph.components() {
                if let Anchor::Offset { x, y } = component.anchor {
                    if !components.insert((component.glyph, x, y)) {
                        let msg = format!(
                            "{}: duplicate component {} at {},{}. Duplicate components may cause rendering issues.",
                            ttf.glyph_name_for_id_synthesise(gid),
                            ttf.glyph_name_for_id_synthesise(component.glyph),
                            x,
                            y
                        );
                        let mut status = Status::fail("found-duplicates", &msg);
                        status.add_metadata(Metadata::GlyphProblem {
                            glyph_name: ttf.glyph_name_for_id_synthesise(gid),
                            glyph_id: gid.to_u32(),
                            userspace_location: None,
                            position: Some((x as f32, y as f32)),
                            actual: Some(json!({
                                "component": ttf.glyph_name_for_id_synthesise(component.glyph),
                                "duplicate_at": [x, y]
                            })),
                            expected: Some(json!("No duplicate components at same position")),
                            message: msg,
                        });
                        problems.push(status);
                    }
                }
            }
        }
    }
    return_result(problems)
}

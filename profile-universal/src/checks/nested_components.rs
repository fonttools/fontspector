use std::collections::HashMap;

use fontations::skrifa::raw::{
    tables::{
        glyf::{Glyf, Glyph},
        loca::Loca,
    },
    TableProvider,
};
use fontations::skrifa::GlyphId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

use super::transformed_components::decompose_components_impl;

#[check(
    id = "nested_components",
    rationale = "
        There have been bugs rendering variable fonts with nested components.
        Additionally, some static fonts with nested components have been reported
        to have rendering and printing issues.

        For more info, see:
        * https://github.com/fonttools/fontbakery/issues/2961
        * https://github.com/arrowtype/recursive/issues/412
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2961",
    title = "Ensure glyphs do not have components which are themselves components.",
    hotfix = decompose_nested_components
)]
fn nested_components(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let loca = font
        .font()
        .loca(None)
        .map_err(|_| FontspectorError::skip("no-loca", "loca table not found"))?;
    let glyf = font
        .font()
        .glyf()
        .map_err(|_| FontspectorError::skip("no-glyf", "glyf table not found"))?;
    let mut failures = vec![];
    let composite_glyphs: HashMap<GlyphId, _> = font
        .all_glyphs()
        .filter_map(|glyphid| {
            if let Some(Glyph::Composite(composite)) = loca.get_glyf(glyphid, &glyf).ok()? {
                Some((glyphid, composite))
            } else {
                None
            }
        })
        .collect();
    for (glyphid, composite) in composite_glyphs.iter() {
        for component in composite.components() {
            if composite_glyphs.contains_key(&component.glyph.into()) {
                failures.push(font.glyph_name_for_id_synthesise(*glyphid));
                break;
            }
        }
    }
    if failures.is_empty() {
        Ok(Status::just_one_pass())
    } else {
        Ok(Status::just_one_fail(
            "found-nested-components",
            &format!(
                "The following glyphs have components which are themselves component glyphs:\n\n{}",
                bullet_list(context, failures)
            ),
        ))
    }
}

fn get_depth(glyph_id: GlyphId, loca: &Loca, glyf: &Glyf) -> u32 {
    let mut depth = 0;
    let glyph_entry = loca.get_glyf(glyph_id, glyf).ok().flatten();
    if let Some(Glyph::Composite(composite)) = glyph_entry {
        depth = 1 + composite
            .components()
            .map(|component| get_depth(component.glyph.into(), loca, glyf))
            .max()
            .unwrap_or(0)
    }
    depth
}

fn decompose_nested_components(t: &mut Testable) -> FixFnResult {
    let font = testfont!(t);
    let loca = font.font().loca(None)?;
    let glyf = font.font().glyf()?;
    let mut depths = HashMap::new();
    for glyph in font.all_glyphs() {
        depths.insert(glyph, get_depth(glyph, &loca, &glyf));
    }
    // Drop all with depth <2
    depths.retain(|_, depth| *depth > 1);
    // Sort by depth, descending
    let mut sorted_glyphs: Vec<GlyphId> = depths.keys().copied().collect();
    #[allow(clippy::indexing_slicing)] // We know the key is present!
    sorted_glyphs.sort_by_key(|&glyph| depths[&glyph]);
    sorted_glyphs.reverse();

    decompose_components_impl(t, &sorted_glyphs)
}

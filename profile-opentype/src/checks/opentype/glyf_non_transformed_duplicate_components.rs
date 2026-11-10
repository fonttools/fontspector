use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;
use skrifa::{
    raw::{
        tables::glyf::{Anchor, Glyph},
        TableProvider,
    },
    GlyphId,
};
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, glyph_by_name, run_check, set_glyph, test_able,
    };
    use fontspector_checkapi::StatusCode;
    use skrifa::raw::tables::glyf::Anchor;
    use write_fonts::tables::glyf::Glyph;

    #[test]
    fn test_glyf_non_transformed_duplicate_components_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let result = run_check(super::glyf_non_transformed_duplicate_components, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_glyf_non_transformed_duplicate_components_fail() {
        // Give quotedbl's two components the same x,y coordinates; both are
        // 'quotesingle', so they become duplicates.
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        let quotedbl = glyph_by_name(&testable, "quotedbl").unwrap();
        let Glyph::Composite(mut composite) = quotedbl else {
            panic!("expected quotedbl to be a composite glyph");
        };
        for component in composite.components_mut() {
            component.anchor = Anchor::Offset { x: 0, y: 0 };
        }
        set_glyph(&mut testable, "quotedbl", &Glyph::Composite(composite)).unwrap();

        let result = run_check(super::glyf_non_transformed_duplicate_components, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("found-duplicates".to_string()),
        );
    }
}

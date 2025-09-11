use std::collections::HashSet;

use fontations::skrifa::raw::{
    tables::{
        colr::Paint,
        glyf::Glyph::{Composite, Simple},
    },
    TableProvider,
};
use fontations::skrifa::{charmap::MapVariant, GlyphId, MetadataProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, GetSubstitutionMap};
use itertools::Itertools;

#[check(
    id = "unreachable_glyphs",
    rationale = r#" 
        Glyphs are either accessible directly through Unicode codepoints or through
        substitution rules.

        In Color Fonts, glyphs are also referenced by the COLR table. And mathematical
        fonts also reference glyphs via the MATH table.

        Any glyphs not accessible by these means are redundant and serve only
        to increase the font's file size.
    "#,
    proposal = "https://github.com/fonttools/fontbakery/issues/3160",
    title = "Check font contains no unreachable glyphs"
)]
fn unreachable_glyphs(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut glyphs = f.all_glyphs().collect::<HashSet<_>>();
    // cmap
    for (_, gid) in f.font().charmap().mappings() {
        glyphs.remove(&gid);
    }
    // UVS
    for (_, _, map) in f.font().charmap().variant_mappings() {
        match map {
            MapVariant::UseDefault => {}
            MapVariant::Variant(glyph_id) => {
                glyphs.remove(&glyph_id);
            }
        }
    }

    // No math table support yet, working on it...
    // if let Some(Ok(math)) = f.font().math() {}

    if let Ok(colr) = f.font().colr() {
        // COLRv0
        if let Some(Ok(recs)) = colr.layer_records() {
            for rec in recs {
                glyphs.remove(&rec.glyph_id().into());
            }
        }
        // COLRv1
        if let Some(Ok(base_glyph_array)) = colr.base_glyph_records() {
            for rec in base_glyph_array {
                glyphs.remove(&rec.glyph_id().into());
            }
        }
        if let Some(Ok(base_glyph_list)) = colr.base_glyph_list() {
            for rec in base_glyph_list.base_glyph_paint_records() {
                let paint = rec.paint(base_glyph_list.offset_data())?;
                match paint {
                    Paint::Glyph(paint_glyph) => {
                        glyphs.remove(&paint_glyph.glyph_id().into());
                    }
                    Paint::ColrGlyph(table_ref) => {
                        glyphs.remove(&table_ref.glyph_id().into());
                    }
                    _ => {}
                }
                glyphs.remove(&rec.glyph_id().into());
            }
        }
        if let Some(Ok(layer_list)) = colr.layer_list() {
            for rec in layer_list.paints().iter().flatten() {
                match rec {
                    Paint::Glyph(paint_glyph) => {
                        glyphs.remove(&paint_glyph.glyph_id().into());
                    }
                    Paint::ColrGlyph(table_ref) => {
                        glyphs.remove(&table_ref.glyph_id().into());
                    }
                    _ => {}
                }
            }
        }
    }

    // GSUB productions
    if let Ok(gsub) = f.font().gsub() {
        for lookup in gsub.lookup_list()?.lookups().iter().flatten() {
            let substitutions = lookup.subtables()?.substitutions()?;
            for (_lhs, rhs) in substitutions {
                for gid in rhs.iter() {
                    glyphs.remove(&GlyphId::from(*gid));
                }
            }
        }
    }
    // Remove components used in TrueType table
    for glyph in f
        .all_glyphs()
        .flat_map(|gid| f.get_glyf_glyph(gid))
        .flatten()
    {
        match glyph {
            Simple(_) => {}
            Composite(composite) => {
                for component in composite.components() {
                    glyphs.remove(&component.glyph.into());
                }
            }
        }
    }

    glyphs.remove(&GlyphId::from(0u32));

    if glyphs.is_empty() {
        Ok(Status::just_one_pass())
    } else {
        Ok(Status::just_one_warn(
            "unreachable-glyphs",
            &format!(
                "The following glyphs could not be reached by codepoint or substitution rules:\n\n{}",
                bullet_list(
                    context,
                    glyphs
                        .iter()
                        .sorted()
                        .map(|gid| f.glyph_name_for_id_synthesise(*gid))
                )
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_messages_contain, assert_messages_dont_contain, assert_pass, assert_results_contain,
        run_check, test_able,
    };
    use fontspector_checkapi::StatusCode;

    #[test]
    fn test_check_unreachable_glyphs_pass() {
        let testable = test_able("noto_sans_tamil_supplement/NotoSansTamilSupplement-Regular.ttf");
        let results = run_check(super::unreachable_glyphs, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_check_unreachable_glyphs_colrv0_pass() {
        let testable = test_able("color_fonts/AmiriQuranColored.ttf");
        let results = run_check(super::unreachable_glyphs, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_check_unreachable_glyphs_colrv1_pass() {
        let testable = test_able("color_fonts/noto-glyf_colr_1.ttf");
        let results = run_check(super::unreachable_glyphs, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_check_unreachable_glyphs_fail() {
        let testable = test_able("merriweather/Merriweather-Regular.ttf");
        let results = run_check(super::unreachable_glyphs, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("unreachable-glyphs".to_string()),
        );
        let expected_glyphs = vec![
            "Gtilde",
            "eight.dnom",
            "four.dnom",
            "three.dnom",
            "two.dnom",
            "i.dot",
            "five.numr",
            "seven.numr",
            "bullet.cap",
            "periodcentered.cap",
            "ampersand.sc",
            "I.uc",
        ];
        for glyph in expected_glyphs {
            assert_messages_contain(&results, glyph);
        }
        let unexpected_glyphs = vec![
            "caronvertical",
            "acute.cap",
            "breve.cap",
            "caron.cap",
            "circumflex.cap",
            "dotaccent.cap",
            "dieresis.cap",
            "grave.cap",
            "hungarumlaut.cap",
            "macron.cap",
            "ring.cap",
            "tilde.cap",
            "breve.r",
            "breve.rcap",
        ];
        for glyph in unexpected_glyphs {
            assert_messages_dont_contain(&results, glyph);
        }
    }
}

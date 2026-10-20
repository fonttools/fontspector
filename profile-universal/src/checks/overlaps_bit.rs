use std::collections::HashMap;

use fontspector_checkapi::pens::BezGlyph;
use fontspector_checkapi::{prelude::*, TestFont, DEFAULT_LOCATION};
use fontspector_checkapi::{
    return_result, testfont, CheckFnResult, Context, FileTypeConvert, FontspectorError, Testable,
};
use kurbo::BezPath;
use linesweeper::{BinaryOp, FillRule};
use skrifa::instance::Size;
use skrifa::outline::DrawSettings;
use skrifa::raw::TableProvider;
use skrifa::{GlyphId, MetadataProvider};
use write_fonts::from_obj::FromTableRef as _;
use write_fonts::tables::glyf::{GlyfLocaBuilder, Glyph, SimpleGlyph};
use write_fonts::FontBuilder;

pub fn has_overlaps(bez_path: &BezPath) -> Result<bool, FontspectorError> {
    let non_zero_beziers = linesweeper::binary_op(
        bez_path,
        &BezPath::new(),
        FillRule::NonZero,
        BinaryOp::Union,
    )
    .map_err(|e| FontspectorError::General(format!("Linesweeper failed to check overlaps: {e}")))?;
    let even_odd_beziers = linesweeper::binary_op(
        bez_path,
        &BezPath::new(),
        FillRule::EvenOdd,
        BinaryOp::Union,
    )
    .map_err(|e| FontspectorError::General(format!("Linesweeper failed to check overlaps: {e}")))?;

    let has_overlaps = Iterator::ne(
        non_zero_beziers.contours().map(|nzc| &nzc.path),
        even_odd_beziers.contours().map(|eoc| &eoc.path),
    );
    Ok(has_overlaps)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum OverlapProblem {
    HasButShouldNot,
    ShouldHaveButDoesNot,
}

fn classify_glyphs(f: &TestFont) -> Result<HashMap<GlyphId, OverlapProblem>, FontspectorError> {
    let mut results = HashMap::new();

    for gid in f.all_glyphs() {
        let mut pen = BezGlyph::default();
        let Some(glyph) = f.font().outline_glyphs().get(gid) else {
            continue;
        };
        let location = f.font().axes().location(DEFAULT_LOCATION);
        let settings = DrawSettings::unhinted(Size::unscaled(), &location);

        glyph.draw(settings, &mut pen)?;
        let current_overlaps_bit = glyph.has_overlaps().unwrap_or(false);
        // Unify to single BezPath
        let mut path = BezPath::new();
        for pth in pen.0 {
            path.extend(pth);
        }
        if !has_overlaps(&path)? {
            if current_overlaps_bit {
                results.insert(gid, OverlapProblem::HasButShouldNot);
            }
            continue;
        }
        if !current_overlaps_bit {
            results.insert(gid, OverlapProblem::ShouldHaveButDoesNot);
        }
    }
    Ok(results)
}

#[check(
    id = "overlaps_bit",
    rationale = "
        Fonts whose glyphs contain overlapping paths should have the overlaps bit set correctly
        to avoid rendering errors. (See for example https://github.com/google/fonts/issues/11120)

        This should normally be done by the font compiler, but some compilers do not correctly set
        the bit.
    ",
    proposal = "https://github.com/googlefonts/gftools/pulls/965",
    title = "Ensure font has overlaps bit set correctly.",
    hotfix = fix_overlaps_bit,
)]
fn overlaps_bit(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let glyph_classification = classify_glyphs(&f)?;

    let reverse_char_map = f
        .font()
        .charmap()
        .mappings()
        .map(|(cp, gid)| (gid, cp))
        .collect::<std::collections::HashMap<_, _>>();

    let name_for_gid = |gid| {
        let mut name = f.glyph_name_for_id_synthesise(gid);
        if let Some(cp) = reverse_char_map.get(&gid) {
            name = format!("{name} (U+{cp:04X})");
        }
        name
    };

    let mut problems = vec![];
    let bit_should_be_set: Vec<_> = glyph_classification
        .iter()
        .filter_map(|(&gid, &problem)| {
            if problem == OverlapProblem::ShouldHaveButDoesNot {
                Some(gid)
            } else {
                None
            }
        })
        .collect();
    let bit_should_be_clear: Vec<_> = glyph_classification
        .iter()
        .filter_map(|(&gid, &problem)| {
            if problem == OverlapProblem::HasButShouldNot {
                Some(gid)
            } else {
                None
            }
        })
        .collect();
    if !bit_should_be_set.is_empty() {
        problems.push(Status::warn(
            "no-overlaps-bit",
            &format!(
                "The following glyphs have overlaps but the overlaps bit is not set:\n{}",
                bullet_list(
                    context,
                    bit_should_be_set
                        .iter()
                        .map(|gid| name_for_gid(*gid))
                        .collect::<Vec<_>>()
                )
            ),
        ));
    }
    if !bit_should_be_clear.is_empty() {
        problems.push(Status::warn(
            "overlaps-bit-set",
            &format!(
                "The following glyphs do not have overlaps but the overlaps bit is set:\n{}",
                bullet_list(
                    context,
                    bit_should_be_clear
                        .iter()
                        .map(|gid| name_for_gid(*gid))
                        .collect::<Vec<_>>()
                )
            ),
        ));
    }
    return_result(problems)
}

fn fix_overlaps_bit(
    t: &mut Testable,
    _replies: Option<MoreInfoReplies>,
) -> Result<FixResult, FontspectorError> {
    let f = testfont!(t);
    let classification = classify_glyphs(&f)?;
    let loca = f.font().loca(None)?;
    let glyf = f.font().glyf()?;
    let glyph_count: u32 = f.font().maxp()?.num_glyphs().into();
    let mut builder = GlyfLocaBuilder::new();
    for i in 0..glyph_count {
        let gid = GlyphId::from(i);
        if let Ok(Some(g)) = loca.get_glyf(gid, &glyf) {
            let mut glyph = write_fonts::tables::glyf::Glyph::from_table_ref(&g);
            match glyph {
                Glyph::Simple(ref mut simple_glyph) => match classification.get(&gid) {
                    Some(&OverlapProblem::ShouldHaveButDoesNot) => {
                        simple_glyph.overlaps = true;
                    }
                    Some(&OverlapProblem::HasButShouldNot) => {
                        simple_glyph.overlaps = false;
                    }
                    _ => {}
                },
                Glyph::Composite(ref mut _composite_glyph) => match classification.get(&gid) {
                    Some(&OverlapProblem::ShouldHaveButDoesNot) => {
                        // No composite glyph overlaps field in write-fonts
                        // composite_glyph.overlaps = true;
                    }
                    Some(&OverlapProblem::HasButShouldNot) => {
                        // composite_glyph.overlaps = false;
                    }
                    _ => {}
                },
                Glyph::Empty => {}
            }
            builder
                .add_glyph(&glyph)
                .map_err(|e| FontspectorError::Fix(format!("Failed to add glyph: {}", e)))?;
        } else {
            // Push an empty glyph so we don't get out of sync
            builder
                .add_glyph(&Glyph::Simple(SimpleGlyph::default()))
                .map_err(|e| FontspectorError::Fix(format!("Failed to add empty glyph: {}", e)))?;
        }
    }
    let (glyf, loca, _loca_format) = builder.build();
    let mut new_font = FontBuilder::new();
    new_font.add_table(&glyf)?;
    new_font.add_table(&loca)?;
    new_font.copy_missing_tables(f.font());
    t.set(new_font.build());

    Ok(FixResult::Fixed)
}

use fontations::{
    read::{
        tables::base::{Base, BaseScriptList, BaseScriptRecord},
        ReadError,
    },
    skrifa::{
        metrics::BoundingBox,
        prelude::{LocationRef, Size},
        raw::{tables::os2::SelectionFlags, TableProvider},
        MetadataProvider,
    },
    types::{BigEndian, GlyphId, Tag},
};
use fontspector_checkapi::GetSubstitutionMap;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, TestFont};
use hashbrown::{HashMap, HashSet};
use tabled::builder::Builder;

use crate::network_conditions::is_listed_on_google_fonts;

fn close_enough(a: impl Into<f32>, tolerance: f32, expected: f32) -> bool {
    (a.into() - expected).abs() <= tolerance
}

#[derive(Debug, Clone)]
struct CjkMetrics {
    /// Ideographic character face bottom edge
    h_icfb: Option<f32>,
    /// Ideographic character face top edge
    h_icft: Option<f32>,
    /// Ideographic em-box bottom edge
    h_ideo: Option<f32>,
    /// Ideographic em-box top edge
    h_idtp: Option<f32>,
    /// Roman baseline
    h_romn: Option<f32>,

    /// Ideographic character face left edge
    v_icfb: Option<f32>,
    /// Ideographic character face right edge
    v_icft: Option<f32>,
    /// Ideographic em-box left edge
    v_ideo: Option<f32>,
    /// Ideographic em-box right edge (advance width)
    v_idtp: Option<f32>,
    /// Vertical roman baseline
    v_romn: Option<f32>,
}

struct SimpleAxis {
    default_baseline: Tag,
    metrics: CjkMetrics,
}
struct SimpleCjkBaseTable(HashMap<Tag, SimpleAxis>);

impl CjkMetrics {
    fn from_bounds(bounds: &[BoundingBox], upem: f32, average_width: f32) -> Self {
        let bbox_y_average = bounds
            .iter()
            .map(|b| (b.y_max + b.y_min) / 2.0)
            .sum::<f32>()
            / bounds.len() as f32;
        let h_idtp = bbox_y_average + upem / 2.0;
        let h_ideo = bbox_y_average - upem / 2.0;
        let average_top = bounds.iter().map(|b| b.y_max).sum::<f32>() / bounds.len() as f32;
        let average_bottom = bounds.iter().map(|b| b.y_min).sum::<f32>() / bounds.len() as f32;
        let average_left = bounds.iter().map(|b| b.x_min).sum::<f32>() / bounds.len() as f32;
        let average_right = bounds.iter().map(|b| b.x_max).sum::<f32>() / bounds.len() as f32;

        CjkMetrics {
            h_icfb: Some(average_bottom),
            h_icft: Some(average_top),
            h_ideo: Some(h_ideo),
            h_idtp: Some(h_idtp),
            h_romn: Some(0.0),
            v_icfb: Some(average_left),
            v_icft: Some(average_right),
            v_ideo: Some(0.0),
            v_idtp: Some(average_width),
            v_romn: Some(-h_ideo),
        }
    }
}

impl SimpleCjkBaseTable {
    fn from_base(base_table: &Base) -> Result<Self, CheckError> {
        let mut horizontal_collections = HashMap::new();
        let mut vertical_collections = HashMap::new();

        let horiz_axis = base_table.horiz_axis().ok_or(CheckError::Error(
            "BASE table must have a horizontal axis".to_string(),
        ))??;
        let taglist = horiz_axis.base_tag_list().ok_or(CheckError::Error(
            "BASE table must have a horizontal tag list".to_string(),
        ))??;
        let tags = taglist.baseline_tags();
        let script_list = horiz_axis.base_script_list()?;
        for base_script_record in script_list.base_script_records() {
            horizontal_collections.insert(
                base_script_record.base_script_tag(),
                base_script_to_collection(tags, &script_list, base_script_record)?,
            );
        }

        let vert_axis = base_table.vert_axis().ok_or(CheckError::Error(
            "BASE table must have a vertical axis".to_string(),
        ))??;
        let taglist = vert_axis.base_tag_list().ok_or(CheckError::Error(
            "BASE table must have a vertical tag list".to_string(),
        ))??;
        let tags = taglist.baseline_tags();
        let script_list = vert_axis.base_script_list()?;
        for base_script_record in script_list.base_script_records() {
            vertical_collections.insert(
                base_script_record.base_script_tag(),
                base_script_to_collection(tags, &script_list, base_script_record)?,
            );
        }

        let mut table = SimpleCjkBaseTable(HashMap::new());
        for (script_tag, (default_baseline, collection)) in horizontal_collections {
            let v_collection = vertical_collections.get(&script_tag).cloned();
            let metrics = CjkMetrics {
                h_icfb: collection.get(&Tag::new(b"icfb")).map(|v| *v as f32),
                h_icft: collection.get(&Tag::new(b"icft")).map(|v| *v as f32),
                h_ideo: collection.get(&Tag::new(b"ideo")).map(|v| *v as f32),
                h_idtp: collection.get(&Tag::new(b"idtp")).map(|v| *v as f32),
                h_romn: collection.get(&Tag::new(b"romn")).map(|v| *v as f32),
                v_icfb: v_collection
                    .as_ref()
                    .and_then(|(_, c)| c.get(&Tag::new(b"icfb")).map(|v| *v as f32)),
                v_icft: v_collection
                    .as_ref()
                    .and_then(|(_, c)| c.get(&Tag::new(b"icft")).map(|v| *v as f32)),
                v_ideo: v_collection
                    .as_ref()
                    .and_then(|(_, c)| c.get(&Tag::new(b"ideo")).map(|v| *v as f32)),
                v_idtp: v_collection
                    .as_ref()
                    .and_then(|(_, c)| c.get(&Tag::new(b"idtp")).map(|v| *v as f32)),
                v_romn: v_collection
                    .as_ref()
                    .and_then(|(_, c)| c.get(&Tag::new(b"romn")).map(|v| *v as f32)),
            };
            table.0.insert(
                script_tag,
                SimpleAxis {
                    default_baseline,
                    metrics,
                },
            );
        }
        Ok(table)
    }

    fn get_any_metrics(&self) -> Option<CjkMetrics> {
        // We return the first script's metrics, as they should be the same for all scripts
        self.0.values().next().map(|axis| axis.metrics.clone())
    }

    #[allow(clippy::unwrap_used)]
    fn validate(
        &self,
        problems: &mut Vec<Status>,
        computed_bounds: &CjkMetrics,
        average_width: f32,
        upem: f32,
    ) -> Result<(), CheckError> {
        let font_is_square = (average_width - upem).abs() / upem < 0.01;
        for (script_tag, axis) in &self.0 {
            let expected_default = if is_cjk(*script_tag) {
                Tag::new(b"ideo")
            } else {
                Tag::new(b"romn")
            };
            if axis.default_baseline != expected_default {
                problems.push(Status::fail(
                    "bad-default-baseline",
                    &format!(
                        "Default baseline for script {} should be {}, but is {}",
                        script_tag, expected_default, axis.default_baseline
                    ),
                ));
            }
            let metrics = &axis.metrics;
            validate_metric(
                problems,
                script_tag,
                computed_bounds.h_icfb.unwrap(),
                metrics.h_icfb,
                upem,
                "horizontal-icfb",
            );
            validate_metric(
                problems,
                script_tag,
                computed_bounds.h_icft.unwrap(),
                metrics.h_icft,
                upem,
                "horizontal-icft",
            );
            validate_metric(
                problems,
                script_tag,
                computed_bounds.h_ideo.unwrap(),
                metrics.h_ideo,
                upem,
                "horizontal-ideo",
            );
            validate_metric(
                problems,
                script_tag,
                computed_bounds.h_romn.unwrap(),
                metrics.h_romn,
                upem,
                "horizontal-romn",
            );
            validate_metric(
                problems,
                script_tag,
                computed_bounds.v_icfb.unwrap(),
                metrics.v_icfb,
                upem,
                "vertical-icfb",
            );
            validate_metric(
                problems,
                script_tag,
                computed_bounds.v_icft.unwrap(),
                metrics.v_icft,
                upem,
                "vertical-icft",
            );
            validate_metric(
                problems,
                script_tag,
                computed_bounds.v_ideo.unwrap(),
                metrics.v_ideo,
                upem,
                "vertical-ideo",
            );
            validate_metric(
                problems,
                script_tag,
                computed_bounds.v_romn.unwrap(),
                metrics.v_romn,
                upem,
                "vertical-romn",
            );
            if !font_is_square {
                validate_metric(
                    problems,
                    script_tag,
                    computed_bounds.h_idtp.unwrap(),
                    metrics.h_idtp,
                    upem,
                    "horizontal-idtp",
                );
                validate_metric(
                    problems,
                    script_tag,
                    computed_bounds.v_idtp.unwrap(),
                    metrics.v_idtp,
                    upem,
                    "vertical-idtp",
                );
            }
        }
        Ok(())
    }
}

fn validate_metric(
    problems: &mut Vec<Status>,
    script_tag: &Tag,
    expected_value: f32,
    actual_value: Option<f32>,
    upem: f32,
    tag_name: &str,
) {
    if let Some(value) = actual_value {
        if !close_enough(value, 0.05 * upem, expected_value) {
            problems.push(Status::fail(
                &format!("bad-{}", tag_name),
                &format!(
                    "{} for script {} is {:.0}; it should be {:.0} within 5% of upem",
                    tag_name.replace('-', " "),
                    script_tag,
                    value,
                    expected_value
                ),
            ));
        }
    } else {
        problems.push(Status::fail(
            &format!("missing-{}", tag_name),
            &format!(
                "{} for script {} should be present",
                tag_name.replace('-', " "),
                script_tag
            ),
        ));
    }
}

fn base_script_to_collection(
    tags: &[BigEndian<Tag>],
    script_list: &BaseScriptList,
    base_script_record: &BaseScriptRecord,
) -> Result<(Tag, HashMap<Tag, i16>), CheckError> {
    let base_script = base_script_record.base_script(script_list.offset_data())?;
    let values = base_script.base_values().ok_or(CheckError::Error(
        "BASE table must have base values".to_string(),
    ))??;
    let Some(default_baseline) = tags
        .get(values.default_baseline_index() as usize)
        .map(|be| be.get())
    else {
        return Err(CheckError::Error(
            "BASE table must have a default baseline".to_string(),
        ));
    };
    let collection: HashMap<_, _> = tags
        .iter()
        .map(|be| be.get())
        .zip(
            values
                .base_coords()
                .iter()
                .flatten()
                .map(|v| v.coordinate()),
        )
        .collect();
    Ok((default_baseline, collection))
}

fn vertical_glyphs(f: &TestFont) -> Result<HashSet<GlyphId>, ReadError> {
    // Dig in GSUB to find vert/vrt2
    let vert_lookup_indexes: HashSet<usize> = f
        .feature_records(true)
        .filter(|(f, _)| {
            f.feature_tag() == Tag::new(b"vert") || f.feature_tag() == Tag::new(b"vrt2")
        })
        .flat_map(|(_, rec)| rec)
        .flat_map(|rec| rec.lookup_list_indices().iter())
        .map(|x| x.get() as usize)
        .collect();
    let vert_lookups = f
        .font()
        .gsub()?
        .lookup_list()?
        .lookups()
        .iter()
        .enumerate()
        .filter_map(|(index, lookup)| {
            if vert_lookup_indexes.contains(&index) {
                Some(lookup)
            } else {
                None
            }
        })
        .flatten();
    let mut vert_glyphs = HashSet::new();
    for lookup in vert_lookups {
        for (_lefts, rights) in lookup.subtables()?.substitutions()?.iter() {
            vert_glyphs.extend(rights.iter().map(|g| GlyphId::from(*g)));
        }
    }

    Ok(vert_glyphs)
}
fn is_cjk(script_tag: Tag) -> bool {
    script_tag == Tag::new(b"hani")
        || script_tag == Tag::new(b"jpan")
        || script_tag == Tag::new(b"kore")
        || script_tag == Tag::new(b"bopo")
        || script_tag == Tag::new(b"hira")
        || script_tag == Tag::new(b"hang")
        || script_tag == Tag::new(b"jamo")
        || script_tag == Tag::new(b"hant")
        || script_tag == Tag::new(b"kana")
        || script_tag == Tag::new(b"DFLT") // special case, CJK should be default
}

#[check(
    id = "googlefonts/cjk_vertical_metrics",
    rationale = "
        
        CJK fonts have different vertical metrics when compared to Latin fonts.

        Our documentation includes further information:
        https://github.com/googlefonts/gf-docs/tree/main/Spec#cjk-vertical-metrics
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/2797",
    title = "Check font follows the Google Fonts CJK vertical metric schema"
)]
fn cjk_vertical_metrics(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let family_name = f
        .best_familyname()
        .ok_or(CheckError::Error("Font lacks a family name".to_string()))?;
    if !context.skip_network {
        skip!(
            is_listed_on_google_fonts(&family_name, context).map_err(CheckError::Error)?,
            "already-onboarded",
            "Not checking vertical metrics for fonts already onboarded to Google Fonts"
        );
    }
    skip!(
        !f.is_cjk_font(Some(context)),
        "non-cjk",
        "Not checking non-CJK fonts"
    );
    let metrics = f.vertical_metrics()?;
    let glyph_metrics = f
        .font()
        .glyph_metrics(Size::unscaled(), LocationRef::default());

    let mut cjk_glyphs = f
        .cjk_codepoints(Some(context))
        .filter(|cp| {
            // We're going to be using this to find the ideographic bounding
            // box, so we're only interesting in Han/Kanji. In some designs,
            // kana, enclosed characters, etc. may be taller than the
            // ideographic bounding box, so we exclude them.
            (0x4E00..0x9FFF).contains(cp)
            || (0x3400..0x4DBF).contains(cp) // CJK Unified Ideographs Extension A
            || (0x20000..0x2A6DF).contains(cp) // CJK Unified Ideographs Extension B
        })
        .flat_map(|cp| f.font().charmap().map(cp))
        .collect::<Vec<_>>();
    if cjk_glyphs.is_empty() {
        // Maybe just a Korean or Kana font?
        cjk_glyphs = f
            .cjk_codepoints(Some(context))
            .filter(|cp| {
                // Korean Hangul syllables
                (0xAC00..=0xD7AF).contains(cp)
                || // Kana characters
                (0x3040..=0x30FF).contains(cp)
                || (0xFF00..=0xFFEF).contains(cp) // Full-width Kana
            })
            .flat_map(|cp| f.font().charmap().map(cp))
            .collect();
    }
    skip!(
        cjk_glyphs.is_empty(),
        "no-cjk-glyphs",
        "No CJK glyphs found in the font"
    );

    let actual_base = if f.has_table(b"BASE") {
        let base = f.font().base()?;
        Some(SimpleCjkBaseTable::from_base(&base)?)
    } else {
        None
    };
    let actual_bounds = actual_base.as_ref().and_then(|b| b.get_any_metrics());

    let upem = f.font().head()?.units_per_em() as f32;
    let hmtx = f.font().hmtx()?;
    let average_width = cjk_glyphs
        .iter()
        .map(|&gid| hmtx.advance(gid).map(|x| x as f32).unwrap_or(upem)) // Promote to f32 to avoid overflow
        .sum::<f32>()
        / cjk_glyphs.len() as f32;
    let computed_bounds = CjkMetrics::from_bounds(
        &cjk_glyphs
            .iter()
            .filter_map(|&gid| glyph_metrics.bounds(gid))
            .collect::<Vec<_>>(),
        upem,
        average_width,
    );

    problems.push(Status::info(
        "computed-bounds",
        &format!(
            "Computed BASE table entries: \n\n{}",
            comparison_base_table(&computed_bounds, &actual_bounds)
        ),
    ));

    // Our conditions are:
    // OS/2.fsSelection bit 7 (Use_Typo_Metrics) should be set
    if !f
        .font()
        .os2()?
        .fs_selection()
        .contains(SelectionFlags::USE_TYPO_METRICS)
    {
        problems.push(Status::fail(
            "bad-fselection-bit7",
            "OS/2 fsSelection bit 7 must be enabled",
        ));
    }

    // OS/2.sTypoAscender should be between ideoEmBoxTop + (5%-10% * em-box)
    // i.e. ideoEmBoxTop + 0.075 * upem +/- 0.025 * upem
    #[allow(clippy::unwrap_used)]
    let expected_os2_ascender = actual_bounds
        .as_ref()
        .and_then(|b| b.h_idtp)
        .unwrap_or(computed_bounds.h_idtp.unwrap())
        + 0.075 * upem;

    if !close_enough(
        metrics.os2_typo_ascender,
        0.025 * upem,
        expected_os2_ascender,
    ) {
        problems.push(Status::fail(
            "bad-OS/2.sTypoAscender",
            &format!(
                "OS/2.sTypoAscender is {}; it should be between {:.0} and {:.0}",
                metrics.os2_typo_ascender,
                expected_os2_ascender - 0.025 * upem,
                expected_os2_ascender + 0.025 * upem
            ),
        ));
    }

    // prefer BASE table if available
    #[allow(clippy::unwrap_used)]
    let expected_os2_descender = actual_bounds
        .as_ref()
        .and_then(|b| b.h_ideo)
        .unwrap_or(computed_bounds.h_ideo.unwrap())
        - 0.075 * upem;

    if !close_enough(
        metrics.os2_typo_descender,
        0.025 * upem,
        expected_os2_descender,
    ) {
        problems.push(Status::fail(
            "bad-OS/2.sTypoDescender",
            &format!(
                "OS/2.sTypoDescender is {}; it should be between {:.0} and {:.0}",
                metrics.os2_typo_descender,
                expected_os2_descender - 0.025 * upem,
                expected_os2_descender + 0.025 * upem
            ),
        ));
    }

    // OS/2.sTypoLineGap should be 0
    if metrics.os2_typo_linegap != 0 {
        problems.push(Status::fail(
            "bad-OS/2.sTypoLineGap",
            &format!(
                "OS/2.sTypoLineGap is {}; it should be 0",
                metrics.os2_typo_linegap
            ),
        ));
    }

    // hhea.lineGap should be 0
    if metrics.hhea_linegap != 0 {
        problems.push(Status::fail(
            "bad-hhea.lineGap",
            &format!("hhea.lineGap is {}; it should be 0", metrics.hhea_linegap),
        ));
    }

    // OS/2.usWinAscent	should be font bbox yMax / yMin, but excluding any vertical glyphs
    let verts = vertical_glyphs(&f)?;
    let all_bboxes = f
        .all_glyphs()
        .filter(|&gid| !verts.contains(&gid))
        .filter_map(|gid| {
            f.font()
                .glyph_metrics(Size::unscaled(), LocationRef::default())
                .bounds(gid)
        })
        .map(|b| (b.y_max, b.y_min))
        .collect::<Vec<_>>();
    if let Some(bbox_ymax) = all_bboxes
        .iter()
        .map(|(mx, _mn)| *mx)
        .max_by(f32::total_cmp)
    {
        if metrics.os2_win_ascent as f32 != bbox_ymax {
            problems.push(Status::fail(
                "bad-OS/2.usWinAscent",
                &format!(
                    "OS/2.usWinAscent is {}; it should be {}",
                    metrics.os2_win_ascent, bbox_ymax
                ),
            ));
        }
    }

    // OS/2.usWinDescent should be absolute value of font bbox yMin
    if let Some(bbox_ymin) = all_bboxes
        .iter()
        .map(|(_mx, mn)| *mn)
        .min_by(f32::total_cmp)
    {
        if metrics.os2_win_descent as f32 != bbox_ymin.abs() {
            problems.push(Status::fail(
                "bad-OS/2.usWinDescent",
                &format!(
                    "OS/2.usWinDescent is {}; it should be {}",
                    metrics.os2_win_descent,
                    bbox_ymin.abs()
                ),
            ));
        }
    }

    // hhea.ascender should match OS/2.sTypoAscender
    if metrics.hhea_ascent != metrics.os2_typo_ascender {
        problems.push(Status::fail(
            "ascent-mismatch",
            "hhea.ascent must match OS/2.sTypoAscender",
        ));
    }

    // hhea.descender should match absolute value of OS/2.sTypoDescender
    if metrics.hhea_descent.unsigned_abs() != metrics.os2_typo_descender.unsigned_abs() {
        problems.push(Status::fail(
            "descent-mismatch",
            "hhea.descent must match absolute value of OS/2.sTypoDescender",
        ));
    }

    // A BASE table with correct icfb/icft/ideo/romn baselines should be present
    if let Some(base) = actual_base {
        base.validate(&mut problems, &computed_bounds, average_width, upem)?
    } else {
        let font_is_square = (average_width - upem).abs() / upem < 0.01;

        problems.push(Status::fail(
            "missing-BASE-table",
            format!(
                "A BASE table with correct icfb/icft/ideo/romn{} baselines should be present",
                if font_is_square { "" } else { " and idtp" }
            )
            .as_str(),
        ));
    }

    // vmtx and vhea tables should be present (and correct)
    if !f.has_table(b"vmtx") {
        problems.push(Status::fail(
            "missing-vmtx-table",
            "A vmtx table should be present",
        ));
    }
    if !f.has_table(b"vhea") {
        problems.push(Status::fail(
            "missing-vhea-table",
            "A vhea table should be present",
        ));
    }

    return_result(problems)
}

fn comparison_base_table(
    computed_bounds: &CjkMetrics,
    actual_bounds: &Option<CjkMetrics>,
) -> String {
    let mut table = Builder::new();
    table.push_column(vec![
        "Baseline",
        "Horizontal icfb",
        "Horizontal icft",
        "Horizontal ideo",
        "Horizontal idtp",
        "Horizontal romn",
        "Vertical icfb",
        "Vertical icft",
        "Vertical ideo",
        "Vertical idtp",
        "Vertical romn",
    ]);
    table.push_column(vec![
        "Computed".to_string(),
        computed_bounds
            .h_icfb
            .map_or("N/A".to_string(), |v| (v as i32).to_string()),
        computed_bounds
            .h_icft
            .map_or("N/A".to_string(), |v| (v as i32).to_string()),
        computed_bounds
            .h_ideo
            .map_or("N/A".to_string(), |v| (v as i32).to_string()),
        computed_bounds
            .h_idtp
            .map_or("N/A".to_string(), |v| (v as i32).to_string()),
        computed_bounds
            .h_romn
            .map_or("N/A".to_string(), |v| (v as i32).to_string()),
        computed_bounds
            .v_icfb
            .map_or("N/A".to_string(), |v| (v as i32).to_string()),
        computed_bounds
            .v_icft
            .map_or("N/A".to_string(), |v| (v as i32).to_string()),
        computed_bounds
            .v_ideo
            .map_or("N/A".to_string(), |v| (v as i32).to_string()),
        computed_bounds
            .v_idtp
            .map_or("N/A".to_string(), |v| (v as i32).to_string()),
        computed_bounds
            .v_romn
            .map_or("N/A".to_string(), |v| (v as i32).to_string()),
    ]);
    if let Some(bounds) = actual_bounds {
        table.push_column(vec![
            "BASE table".to_string(),
            bounds.h_icfb.map_or("N/A".to_string(), |v| v.to_string()),
            bounds.h_icft.map_or("N/A".to_string(), |v| v.to_string()),
            bounds.h_ideo.map_or("N/A".to_string(), |v| v.to_string()),
            bounds.h_idtp.map_or("N/A".to_string(), |v| v.to_string()),
            bounds.h_romn.map_or("N/A".to_string(), |v| v.to_string()),
            bounds.v_icfb.map_or("N/A".to_string(), |v| v.to_string()),
            bounds.v_icft.map_or("N/A".to_string(), |v| v.to_string()),
            bounds.v_ideo.map_or("N/A".to_string(), |v| v.to_string()),
            bounds.v_idtp.map_or("N/A".to_string(), |v| v.to_string()),
            bounds.v_romn.map_or("N/A".to_string(), |v| v.to_string()),
        ]);
    }
    let mut table = table.build();
    table.with(tabled::settings::Style::markdown());
    table.to_string()
}

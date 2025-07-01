use std::collections::HashMap;

use fontations::skrifa::{outline::OutlinePen, raw::TableProvider, GlyphId, MetadataProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, DEFAULT_LOCATION};
use serde::Serialize;
use serde_json::Value;

use super::close_but_not_on;
const ALIGNMENT_MISS_EPSILON: i16 = 2; // Four point lee-way on alignment misses

#[derive(Serialize)]
struct Warning {
    glyph_name: String,
    glyph_id: u32,
    x: f32,
    y: f32,
    line: String,
    y_expected: f32,
}

struct AlignmentMissPen<'a> {
    glyph_id: GlyphId,
    glyph_name: &'a str,
    is_uppercase: bool,
    alignments: &'a HashMap<String, i16>,
    epsilon: i16,
    warnings: Vec<Warning>,
}

impl AlignmentMissPen<'_> {
    fn update(&mut self, x: f32, y: f32) {
        for (line, &y_expected) in self.alignments {
            if line == "x-height" && self.is_uppercase {
                continue;
            }
            if close_but_not_on(y_expected, y as i16, self.epsilon) {
                self.warnings.push(Warning {
                    glyph_name: self.glyph_name.to_string(),
                    x,
                    y,
                    line: line.to_string(),
                    y_expected: y_expected.into(),
                    glyph_id: self.glyph_id.to_u32(),
                });
            }
        }
    }
}

impl OutlinePen for AlignmentMissPen<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.update(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.update(x, y);
    }

    fn quad_to(&mut self, _cx0: f32, _cy0: f32, x: f32, y: f32) {
        self.update(x, y);
    }

    fn curve_to(&mut self, _cx0: f32, _cy0: f32, _cx1: f32, _cy1: f32, x: f32, y: f32) {
        self.update(x, y);
    }

    fn close(&mut self) {}
}

#[check(
    id = "outline_alignment_miss",
    rationale = "
        
        This check heuristically looks for on-curve points which are close to, but
        do not sit on, significant boundary coordinates. For example, a point which
        has a Y-coordinate of 1 or -1 might be a misplaced baseline point. As well as
        the baseline, here we also check for points near the x-height (but only for
        lowercase Latin letters), cap-height, ascender and descender Y coordinates.

        Not all such misaligned curve points are a mistake, and sometimes the design
        may call for points in locations near the boundaries. As this check is liable
        to generate significant numbers of false positives, it will pass if there are
        more than 100 reported misalignments.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3088",
    title = "Are there any misaligned on-curve points?"
)]
fn alignment_miss(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let mut alignments: HashMap<String, i16> = HashMap::new();

    let os2 = f.font().os2()?;
    alignments.insert("baseline".to_string(), 0);
    alignments.insert("ascender".to_string(), os2.s_typo_ascender());
    alignments.insert("descender".to_string(), os2.s_typo_descender());
    if let Some(xheight) = os2.sx_height() {
        alignments.insert("x-height".to_string(), xheight);
    }
    if let Some(capheight) = os2.s_cap_height() {
        alignments.insert("cap-height".to_string(), capheight);
    } else {
        problems.push(Status::warn("skip-cap-x-height-alignment",
                &format!("x-height and cap-height checks are skipped because OS/2 table version is only {} and version >= 2 is required for those checks."
                ,os2.version())));
    }
    let mut all_warnings = vec![];
    for glyph in f.all_glyphs() {
        let mut name = f.glyph_name_for_id_synthesise(glyph);
        if let Some((cp, _gid)) = f
            .font()
            .charmap()
            .mappings()
            .find(|(_cp, gid)| *gid == glyph)
        {
            name = format!("{name} (U+{cp:04X})");
        }
        let mut pen = AlignmentMissPen {
            is_uppercase: name.len() > 1 || name.to_uppercase() == name,
            alignments: &alignments,
            epsilon: ALIGNMENT_MISS_EPSILON,
            warnings: vec![],
            glyph_name: &name,
            glyph_id: glyph,
        };
        f.draw_glyph(glyph, &mut pen, DEFAULT_LOCATION)?;
        all_warnings.extend(pen.warnings);
        if all_warnings.len() > 100 {
            problems.push(Status::pass(
                // "skip-many-misalignments",
                // "So many Y-coordinates of points were close to boundaries that this was probably by design.",
            ));
            return return_result(problems);
        }
    }
    if !all_warnings.is_empty() {
        let mut warn = Status::warn(
            "found-misalignments",
            &format!(
                "The following glyphs have on-curve points which have potentially incorrect y coordinates:\n\n{}",
                bullet_list(context, all_warnings.iter().map(|warning| {
                    format!(
                        "- {}: X={},Y={} (should be at {} {}?)",
                        warning.glyph_name, warning.x, warning.y, warning.line, warning.y_expected
                    )
                }))
            ),
        );
        warn.metadata = Some(
            all_warnings
                .iter()
                .map(|warning| {
                    #[allow(clippy::unwrap_used)] // How can it go wrong?
                    serde_json::to_value(warning).unwrap()
                })
                .collect::<Vec<Value>>()
                .into(),
        );
        problems.push(warn);
    }

    return_result(problems)
}

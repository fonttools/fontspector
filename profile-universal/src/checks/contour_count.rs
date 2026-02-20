use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use fontations::skrifa::{GlyphId, MetadataProvider};
use fontspector_checkapi::{
    pens::ContourCountPen, prelude::*, testfont, FileTypeConvert, Metadata, DEFAULT_LOCATION,
};
use serde_json::json;

const DATA_JSON: &str = include_str!("../../data/desired_glyph_data.json");

#[allow(clippy::unwrap_used)]
static GLYPHS_BY_NAME: LazyLock<HashMap<String, HashSet<usize>>> = LazyLock::new(|| {
    let data: serde_json::Map<String, serde_json::Value> = serde_json::from_str(DATA_JSON).unwrap();
    let value = data.get("by_name").unwrap().as_object().unwrap();
    let mut map = HashMap::new();
    for (name, indices) in value {
        let indices = indices.as_array().unwrap();
        let indices = indices
            .iter()
            .map(|v| v.as_u64().unwrap() as usize)
            .collect();
        map.insert(name.clone(), indices);
    }
    map
});

#[allow(clippy::unwrap_used)]
static GLYPHS_BY_UNICODE: LazyLock<HashMap<u32, HashSet<usize>>> = LazyLock::new(|| {
    let data: serde_json::Map<String, serde_json::Value> = serde_json::from_str(DATA_JSON).unwrap();
    let value = data.get("by_unicode").unwrap().as_object().unwrap();
    let mut map = HashMap::new();
    for (codepoint, indices) in value {
        let indices = indices.as_array().unwrap();
        let indices = indices
            .iter()
            .map(|v| v.as_u64().unwrap() as usize)
            .collect();
        map.insert(codepoint.parse::<u32>().unwrap(), indices);
    }
    map
});

#[check(
    id = "contour_count",
    rationale = "
        
        Visually QAing thousands of glyphs by hand is tiring. Most glyphs can only
        be constructured in a handful of ways. This means a glyph's contour count
        will only differ slightly amongst different fonts, e.g a 'g' could either
        be 2 or 3 contours, depending on whether its double story or single story.

        However, a quotedbl should have 2 contours, unless the font belongs
        to a display family.

        This check currently does not cover variable fonts because there's plenty
        of alternative ways of constructing glyphs with multiple outlines for each
        feature in a VarFont. The expected contour count data for this check is
        currently optimized for the typical construction of glyphs in static fonts.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Check if each glyph has the recommended amount of contours."
)]
fn contour_count(t: &Testable, context: &Context) -> CheckFnResult {
    struct ContourIssue {
        glyph_id: GlyphId,
        glyph_name: String,
        codepoint: Option<u32>,
        count: usize,
        expected: Vec<usize>,
    }

    let f = testfont!(t);
    let mut problems = vec![];
    let mut bad_glyphs: Vec<ContourIssue> = vec![];
    let mut zero_contours: Vec<ContourIssue> = vec![];
    let reverse_map = f
        .font()
        .charmap()
        .mappings()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<_, _>>();
    for glyph in f.all_glyphs() {
        let codepoint = reverse_map.get(&glyph).copied();
        let glyph_name = f.glyph_name_for_id_synthesise(glyph);
        let expected = match codepoint {
            Some(cp) => GLYPHS_BY_UNICODE.get(&cp),
            None => GLYPHS_BY_NAME.get(&glyph_name),
        };
        if let Some(data) = expected {
            let mut pen = ContourCountPen::new();
            f.draw_glyph(glyph, &mut pen, DEFAULT_LOCATION)?;
            let count = pen.contour_count();
            let mut expected_counts: Vec<usize> = data.iter().copied().collect();
            expected_counts.sort_unstable();
            let issue = ContourIssue {
                glyph_id: glyph,
                glyph_name: glyph_name.clone(),
                codepoint,
                count,
                expected: expected_counts,
            };
            if count == 0 && !data.contains(&count) {
                zero_contours.push(issue);
            } else if !data.contains(&count) {
                bad_glyphs.push(issue);
            }
        }
    }
    if !bad_glyphs.is_empty() {
        let bad_messages: Vec<String> = bad_glyphs
            .iter()
            .map(|issue| match issue.codepoint {
                Some(cp) => format!(
                    "{} (U+{cp:04X}): found {}, expected one of: {:?}",
                    issue.glyph_name, issue.count, issue.expected
                ),
                None => format!(
                    "{} (unencoded): found {}, expected one of: {:?}",
                    issue.glyph_name, issue.count, issue.expected
                ),
            })
            .collect();
        problems.push(Status::warn(
            "contour-count",
            &format!(
                "This check inspects the glyph outlines and detects the total number of contours in each of them. The expected values are
     inferred from the typical amounts of contours observed in a
     large collection of reference font families. The divergences
     listed below may simply indicate a significantly different
     design on some of your glyphs. On the other hand, some of these
     may flag actual bugs in the font such as glyphs mapped to an
     incorrect codepoint. Please consider reviewing the design and
     codepoint assignment of these to make sure they are correct.\n\n
    The following glyphs do not have the recommended number of contours:\n{}",
                bullet_list(context, &bad_messages),
            ),
        ));
        if let Some(status) = problems.last_mut() {
            for issue in bad_glyphs {
                status.add_metadata(Metadata::GlyphProblem {
                    glyph_name: issue.glyph_name,
                    glyph_id: issue.glyph_id.to_u32(),
                    userspace_location: None,
                    position: None,
                    actual: Some(json!({
                        "contour_count": issue.count,
                        "codepoint": issue.codepoint,
                    })),
                    expected: Some(json!({ "allowed_counts": issue.expected })),
                    message: "Unexpected contour count".to_string(),
                });
            }
        }
    }
    if !zero_contours.is_empty() {
        let zero_messages: Vec<String> = zero_contours
            .iter()
            .map(|issue| match issue.codepoint {
                Some(cp) => format!(
                    "{} (U+{cp:04X}): found {}, expected one of: {:?}",
                    issue.glyph_name, issue.count, issue.expected
                ),
                None => format!(
                    "{} (unencoded): found {}, expected one of: {:?}",
                    issue.glyph_name, issue.count, issue.expected
                ),
            })
            .collect();
        problems.push(Status::fail(
            "no-contour",
            &format!(
                "The following glyphs have no contours even though they were expected to have some:\n{}",
                bullet_list(context, &zero_messages),
            ),
        ));
        if let Some(status) = problems.last_mut() {
            for issue in zero_contours {
                status.add_metadata(Metadata::GlyphProblem {
                    glyph_name: issue.glyph_name,
                    glyph_id: issue.glyph_id.to_u32(),
                    userspace_location: None,
                    position: None,
                    actual: Some(json!({
                        "contour_count": issue.count,
                        "codepoint": issue.codepoint,
                    })),
                    expected: Some(json!({ "min_contours": 1 })),
                    message: "No contours found".to_string(),
                });
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    use super::*;

    #[test]
    fn test_contour_count() {
        let test_font = test_able("ibmplexsans-vf/IBMPlexSansVar-Roman.ttf");
        let result = run_check(contour_count, test_font);
        assert_pass(&result);
    }
}

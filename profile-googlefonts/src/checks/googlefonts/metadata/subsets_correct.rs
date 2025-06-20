use std::collections::{HashMap, HashSet};

use crate::checks::googlefonts::metadata::family_proto;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use google_fonts_subsets::{LATIN, SUBSETS};

const COMMON_CODEPOINTS: [u32; 10] = [
    0x0000, 0x000D, 0x0020, 0x002D, 0x00A0, 0x25CC, 0x200C, 0x200D, 0x0964, 0x0965,
];

const CJK_SUBSETS: [&str; 5] = [
    "chinese-hongkong",
    "chinese-simplified",
    "chinese-traditional",
    "korean",
    "japanese",
];

fn coverage_required(subset: &str) -> f32 {
    if subset.ends_with("-ext") {
        return 0.2;
    }
    match subset {
        "math" | "symbols" => 0.5, // These are listed as "relaxed" subsets in gftools-add-font but seem to have the same defaults?
        "devanagari" => 0.4,       // There are many vedic marks which mess up the percentage
        _ => 0.5,
    }
}

fn support_percentage(
    subset_name: &str,
    subset_codepoints: &HashSet<u32>,
    font_codepoints: &HashSet<u32>,
) -> f32 {
    let subset_codepoints: HashSet<u32> = subset_codepoints
        .iter()
        .filter(|cp| {
            if subset_name == "khmer" {
                // Remove latin from khmer
                !LATIN.contains(cp)
            } else {
                true
            }
        })
        .copied()
        .collect();
    let covered = subset_codepoints.intersection(font_codepoints).count() as f32;
    covered / subset_codepoints.len() as f32
}

#[check(
    id="googlefonts/metadata/subsets_correct",
    rationale="
        The subsets fields in METADATA.pb must not contain any subsets
        for which the font has zero codepoints, and should contain all
        the subsets which the font does support. 'menu' and 'latin' should
        be declared for all fonts, there should be be at most one CJK
        subset declared, and the subsets must appear in alphabetical order.
    ",
    applies_to = "MDPB",
    proposal="https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    title="METADATA.pb subsets are correct?",
    implementation = "all"
)]
fn subsets_correct(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let msg = family_proto(mdpb)?;
    let fonts = msg
        .fonts
        .iter()
        .flat_map(|f| f.filename.as_ref())
        .flat_map(|f| c.get_file(f))
        .collect::<Vec<&Testable>>();
    if fonts.is_empty() {
        skip!("no-fonts", "No font files found in METADATA.pb");
    }
    let subsets = msg.subsets;
    let mut problems = vec![];

    // Let's get our SUBSETS constant into a useful format.
    let google_subsets: HashMap<String, HashSet<u32>> = SUBSETS
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.iter()
                    .copied()
                    .filter(|cp| !COMMON_CODEPOINTS.contains(cp))
                    .collect(),
            )
        })
        .collect();

    // Old menu_and_latin check
    let latin = "latin".to_string();
    let menu = "menu".to_string();
    if !subsets.contains(&latin) && !subsets.contains(&menu) {
        problems.push(Status::fail(
            "missing",
            "Subsets \"menu\" and \"latin\" are mandatory, but but METADATA.pb is missing both",
        ));
    } else if !subsets.contains(&latin) {
        problems.push(Status::fail(
            "missing",
            "Subsets \"menu\" and \"latin\" are mandatory, but but METADATA.pb is missing latin",
        ));
    } else if !subsets.contains(&menu) {
        problems.push(Status::fail(
            "missing",
            "Subsets \"menu\" and \"latin\" are mandatory, but but METADATA.pb is missing menu",
        ));
    }

    // Old subsets_order check
    let mut sorted_subsets = subsets.clone();
    sorted_subsets.sort();
    if subsets != sorted_subsets {
        problems.push(Status::fail("not-sorted", "Subsets are not in order"))
    }

    // Old single_cjk_subset check
    let cjk_subsets = subsets.iter().filter(|s| CJK_SUBSETS.contains(&s.as_str()));
    if cjk_subsets.count() > 1 {
        problems.push(Status::error(
            Some("multiple-cjk-subsets"),
            &format!("METADATA.pb file contains more than one CJK subset. Please choose only one from {}.",
            CJK_SUBSETS.join(", "))
        ));
    }

    // Calculate actual subset for representative font
    #[allow(clippy::indexing_slicing)] // we know it's not empty
    let first_font = fonts[0];
    let ttf = testfont!(first_font);
    let codepoints = ttf.codepoints(Some(context));
    let supported_percentage: HashMap<String, f32> = google_subsets
        .iter()
        .map(|(k, v)| (k.to_string(), support_percentage(k, v, &codepoints)))
        .collect();
    for (name, percentage) in supported_percentage.into_iter() {
        if percentage >= coverage_required(&name) && !subsets.contains(&name) {
            problems.push(Status::warn(
                    "missing-subset",
                    &format!(
                        "Please add '{name}' to METADATA.pb since more than {}% of its glyphs are supported by this font file.",
                        coverage_required(&name) * 100.0
                    ),
                ));
        }
        if percentage < coverage_required(&name) && subsets.contains(&name) {
            if percentage == 0.0 {
                problems.push(Status::fail(
                    "unsupported-subset",
                    &format!(
                        "Please remove '{name}' from METADATA.pb since none of its glyphs are supported by this font file.",
                    ),
                ));
            } else {
                problems.push(Status::warn(
                    "unsupported-subset",
                    &format!(
                        "Please remove '{name}' from METADATA.pb since less than {}% of its glyphs are supported by this font file.",
                        coverage_required(&name) * 100.0
                    ),
                ));
            }
        }
        // if percentage < coverage_required(&name) && percentage > 0.1 && !subsets.contains(&name) {
        //     problems.push(Status::warn(
        //         "barely-supported-subset",
        //         &format!(
        //             "'{}' might need to be added to METADATA.pb; we require more than {}% of its codepoints to be supported by this font, but the font only supports {}% of {} codepoints.",
        //             name,
        //             coverage_required(&name) * 100.0,
        //             percentage * 100.0,
        //             name,
        //         ),
        //     ));
        // }
    }
    for subset in subsets.iter() {
        if !google_subsets.contains_key(subset) && subset != "menu" {
            problems.push(Status::fail(
                "unknown-subset",
                &format!(
                    "Please remove the unrecognized subset '{subset}' from the METADATA.pb file."
                ),
            ));
        }
    }

    return_result(problems)
}

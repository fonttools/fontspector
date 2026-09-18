use std::collections::{HashMap, HashSet};

use crate::{checks::googlefonts::metadata::family_proto, network_conditions::production_metadata};
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
    let local_subsets = msg.subsets.clone();
    let mut problems = vec![];

    // Check production subsets first, since we can't remove them.
    let mut production_subsets: Vec<String> = vec![];
    if !context.skip_network {
        let production_metadata = production_metadata(context).map_err(|e| {
            FontspectorError::General(format!("Failed to fetch production metadata: {e:?}"))
        })?;
        if let Some(subsets_array) = production_metadata
            .get("familyMetadataList")
            .ok_or_else(|| {
                FontspectorError::General(
                    "Failed to get familyMetadataList from production metadata".to_string(),
                )
            })?
            .as_array()
            .ok_or_else(|| {
                FontspectorError::General("familyMetadataList is not an array".to_string())
            })?
            .iter()
            .find(|i| i.get("family").and_then(|f| f.as_str()) == Some(msg.name()))
            .and_then(|i| i.get("subsets"))
            .and_then(|s| s.as_array())
        {
            production_subsets.extend(
                subsets_array
                    .iter()
                    .flat_map(|i| i.as_str().map(|x| x.to_string())),
            );
        };
        let missing_subsets = production_subsets
            .iter()
            .filter(|s| !local_subsets.contains(&s.to_string()))
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !missing_subsets.is_empty() {
            problems.push(Status::fail(
                "missing-subsets",
                &format!(
                    "The following subsets are missing: {}",
                    missing_subsets.join(", ")
                ),
            ))
        }
    }

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
    if !local_subsets.contains(&latin) && !local_subsets.contains(&menu) {
        problems.push(Status::fail(
            "missing",
            "Subsets \"menu\" and \"latin\" are mandatory, but but METADATA.pb is missing both",
        ));
    } else if !local_subsets.contains(&latin) {
        problems.push(Status::fail(
            "missing",
            "Subsets \"menu\" and \"latin\" are mandatory, but but METADATA.pb is missing latin",
        ));
    } else if !local_subsets.contains(&menu) {
        problems.push(Status::fail(
            "missing",
            "Subsets \"menu\" and \"latin\" are mandatory, but but METADATA.pb is missing menu",
        ));
    }

    // Old subsets_order check
    let mut sorted_subsets = local_subsets.clone();
    sorted_subsets.sort();
    if local_subsets != sorted_subsets {
        problems.push(Status::fail("not-sorted", "Subsets are not in order"))
    }

    // Old single_cjk_subset check
    let cjk_subsets = local_subsets
        .iter()
        .filter(|s| CJK_SUBSETS.contains(&s.as_str()));
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
        if percentage >= coverage_required(&name) && !local_subsets.contains(&name) {
            problems.push(Status::warn(
                    "missing-subset",
                    &format!(
                        "Please add '{name}' to METADATA.pb since more than {}% of its glyphs are supported by this font file.",
                        coverage_required(&name) * 100.0
                    ),
                ));
        }
        if percentage < coverage_required(&name) && local_subsets.contains(&name) {
            if production_subsets.contains(&name) {
                problems.push(Status::info(
                    "unsupported-production-subset",
                    &format!(
                        "The subset '{name}' is listed in production metadata, but only {percentage:.2}% of its glyphs are supported by this font file. We can't remove it, however, since that would cause a regression in production.",
                    ))
                )
            } else if percentage == 0.0 {
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
    for subset in local_subsets.iter() {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::subsets_correct;
    use fontspector_checkapi::{
        codetesting::{
            assert_pass, assert_results_contain, run_check_with_config_and_network, test_able,
        },
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        run_check_with_config_and_network(
            subsets_correct,
            TestableType::Collection(&collection),
            HashMap::new(),
            true,
        )
    }

    fn with_subsets(subsets: &[&str]) -> Testable {
        let mdpb = test_able("cabinvf/METADATA.pb");
        let original = String::from_utf8(mdpb.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in cabinvf METADATA fixture: {e}"));
        let old_subsets = "subsets: \"latin\"\nsubsets: \"latin-ext\"\nsubsets: \"menu\"\nsubsets: \"vietnamese\"";
        let new_subsets = subsets
            .iter()
            .map(|s| format!("subsets: \"{s}\""))
            .collect::<Vec<_>>()
            .join("\n");
        let updated = original.replacen(old_subsets, &new_subsets, 1);
        Testable::new_with_contents("METADATA.pb", updated.into_bytes())
    }

    fn with_metadata(mdpb: Testable) -> Vec<Testable> {
        vec![
            test_able("cabinvf/Cabin[wdth,wght].ttf"),
            test_able("cabinvf/Cabin-Italic[wdth,wght].ttf"),
            mdpb,
        ]
    }

    #[test]
    fn test_check_metadata_menu_and_latin() {
        assert_pass(&run(with_metadata(test_able("cabinvf/METADATA.pb"))));

        let missing_latin = run(with_metadata(with_subsets(&[
            "menu",
            "latin-ext",
            "vietnamese",
        ])));
        assert_results_contain(
            &missing_latin,
            StatusCode::Fail,
            Some("missing".to_string()),
        );

        let missing_menu = run(with_metadata(with_subsets(&[
            "latin",
            "latin-ext",
            "vietnamese",
        ])));
        assert_results_contain(&missing_menu, StatusCode::Fail, Some("missing".to_string()));
    }

    #[test]
    fn test_check_metadata_subsets_order() {
        assert_pass(&run(with_metadata(test_able("cabinvf/METADATA.pb"))));

        let unsorted = run(with_metadata(with_subsets(&[
            "menu",
            "latin",
            "latin-ext",
            "vietnamese",
        ])));
        assert_results_contain(&unsorted, StatusCode::Fail, Some("not-sorted".to_string()));
    }

    #[test]
    fn test_check_metadata_single_cjk_subset() {
        let single_cjk = run(with_metadata(with_subsets(&[
            "latin",
            "latin-ext",
            "menu",
            "vietnamese",
            "korean",
        ])));
        let single_cjk_results = single_cjk
            .as_ref()
            .unwrap_or_else(|| panic!("Expected check result for single CJK subset test"));
        assert!(
            !single_cjk_results
                .subresults
                .iter()
                .any(|r| r.code.as_deref() == Some("multiple-cjk-subsets")),
            "single CJK subset unexpectedly triggered multiple-cjk-subsets"
        );

        let multiple_cjk = run(with_metadata(with_subsets(&[
            "latin",
            "latin-ext",
            "menu",
            "vietnamese",
            "korean",
            "japanese",
        ])));
        assert_results_contain(
            &multiple_cjk,
            StatusCode::Error,
            Some("multiple-cjk-subsets".to_string()),
        );
    }
}

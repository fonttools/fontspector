use std::sync::LazyLock;

use fontspector_checkapi::{prelude::*, FileTypeConvert};
use google_fonts_subsets::{subsets_in_font, SUBSETS};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use crate::checks::googlefonts::metadata::family_proto;

static CODEPOINTS_BY_SUBSET: LazyLock<HashMap<u32, HashSet<&'static str>>> = LazyLock::new(|| {
    let mut subsets_by_cp = HashMap::new();
    for (subset_name, subset_cps) in SUBSETS {
        for cp in subset_cps {
            subsets_by_cp
                .entry(*cp)
                .or_insert_with(HashSet::new)
                .insert(subset_name);
        }
    }
    subsets_by_cp
});

#[check(
    id = "googlefonts/metadata/unreachable_subsetting",
    rationale = "
        
        This check ensures that all encoded glyphs in the font are covered by a
        subset declared in the METADATA.pb. Google Fonts splits the font into
        a set of subset fonts based on the contents of the `subsets` field and
        the subset definitions in the `glyphsets` repository.

        Any encoded glyphs which are not by any of these subset definitions
        will not be served in the subsetted fonts, and so will be unreachable to
        the end user.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4097 and https://github.com/fonttools/fontbakery/pull/4273",
    title = "Check for codepoints not covered by METADATA subsets.",
    implementation = "all"
)]
fn unreachable_subsetting(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let ttfs = TTF.from_collection(c);
    let mut problems = vec![];
    let subsets: Vec<String> = c
        .get_file("METADATA.pb")
        .map(|f| family_proto(f).map(|mdpb| mdpb.subsets).unwrap_or_default())
        .unwrap_or_default();
    for font in ttfs {
        let font_codepoints = font.codepoints(Some(context));
        let font_subsets = if subsets.is_empty() {
            subsets_in_font(&font_codepoints, 50.0, Some(0.01))
                .iter()
                .map(|s| s.to_string())
                .collect()
        } else {
            subsets.clone()
        };
        let mut unreachable = font_codepoints.clone();
        for (subset_name, subset_cps) in SUBSETS {
            if font_subsets.contains(&subset_name.to_string()) {
                unreachable.retain(|cp| !subset_cps.contains(cp));
            }
        }
        if unreachable.is_empty() {
            continue;
        }
        let mut bullets = vec![];

        for codepoint in unreachable.into_iter().sorted() {
            let Some(subsets_for_cp) = CODEPOINTS_BY_SUBSET.get(&codepoint) else {
                continue;
            };
            let message = if subsets_for_cp.is_empty() {
                "not included in any glyphset definition".to_string()
            } else if subsets_for_cp.len() == 1 {
                format!("try adding {}", subsets_for_cp.iter().join(", "))
            } else {
                format!("try adding one of: {}", subsets_for_cp.iter().join(", "))
            };
            let name = char::from_u32(codepoint)
                .and_then(unicode_names2::name)
                .map(|x| x.to_string())
                .unwrap_or_else(|| "".to_string());
            let name = format!("U+{codepoint:04X} {name}");
            bullets.push(format!("{name}: {message}"));
        }
        problems.push(Status::warn(
            "unreachable-subsetting",
            &format!(
                "{}: The following codepoints supported by the font are not covered by any subsets defined in the font's metadata file, and will never be served. You can solve this by either manually adding additional subset declarations to METADATA.pb, or by editing the glyphset definitions.\n\n{}\n\nOr you can add the above codepoints to one of the subsets supported by the font: {}",
                font.filename.to_string_lossy(),
                bullet_list(context, bullets),
                font_subsets.join(", ")
            ),
        ))
    }

    return_result(problems)
}

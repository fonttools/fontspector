use std::vec;

use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, TestFont};
use google_fonts_axisregistry::build_fvar_instances;
use indexmap::{IndexMap, IndexSet};
use tabled::builder::Builder;

use crate::utils::build_expected_font;

#[check(
    id = "googlefonts/fvar_instances",
    rationale = "
        
        Check a font's fvar instance coordinates comply with our guidelines:
        https://googlefonts.github.io/gf-guide/variable.html#fvar-instances

        This check is skipped for fonts that have a Morph (MORF) axis
        since we allow users to define their own custom instances.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3800",
    title = "Check variable font instances",
    hotfix = fix_fvar_instances,
)]
fn fvar_instances(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    skip!(
        !f.is_variable_font(),
        "is-variable-font",
        "Font is not a variable font"
    );
    skip!(f.has_axis("MORF"), "has-morf", "Font has a MORF axis");
    let expected_font_data = build_expected_font(&f, &[])?;
    let expected_font = TestFont::new_from_data(&t.filename, &expected_font_data).map_err(|e| {
        FontspectorError::General(format!("Couldn't build expected font from data: {e}"))
    })?;
    let instances: IndexMap<String, _> = f.named_instances().collect();
    let expected_instances: IndexMap<String, _> = expected_font.named_instances().collect();
    let mut table = vec![];
    let mut done = IndexSet::new();
    for name in instances.keys().chain(expected_instances.keys()) {
        if done.contains(name) {
            continue;
        }
        done.insert(name);
        let mut row = IndexMap::new();
        row.insert("Name".to_string(), name.to_string());
        if let Some(font_instance) = instances.get(name) {
            row.insert(
                "current".to_string(),
                font_instance
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
        if let Some(expected_instance) = expected_instances.get(name) {
            row.insert(
                "expected".to_string(),
                expected_instance
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
        table.push(row);
    }
    table.sort_by(|a: &IndexMap<String, String>, b| a.get("expected").cmp(&b.get("expected")));
    let expected_names: IndexSet<_> = expected_instances.keys().collect();
    let current_names: IndexSet<_> = instances.keys().collect();
    let missing_names = expected_names
        .difference(&current_names)
        .collect::<Vec<_>>();
    let new_names = current_names
        .difference(&expected_names)
        .collect::<Vec<_>>();
    let same_names = current_names
        .intersection(&expected_names)
        .collect::<Vec<_>>();
    // same_names are the names that are in both current and expected instances, so these are safe to index
    #[allow(clippy::indexing_slicing)]
    let wght_wrong = expected_instances.values().all(|i| i.contains_key("wght"))
        && same_names
            .iter()
            .any(|i| instances[**i]["wght"] != expected_instances[**i]["wght"]);
    let mut md_table = Builder::from_iter(table.iter().map(|ix| {
        vec![
            ix.get("Name").map_or("Unknown", |s| s.as_ref()),
            ix.get("current").map_or("N/A", |s| s.as_ref()),
            ix.get("expected").map_or("N/A", |s| s.as_ref()),
        ]
    }));
    md_table.insert_record(0, vec!["Name", "current", "expected"]);
    #[allow(clippy::indexing_slicing)]
    if wght_wrong || !missing_names.is_empty() || !new_names.is_empty() {
        let mut hints = vec![];
        if !missing_names.is_empty() {
            hints.push("- Add missing instances");
        }
        if !new_names.is_empty() {
            hints.push("- Delete additional instances");
        }
        if wght_wrong {
            hints.push("- wght coordinates are wrong for some instances");
        }
        problems.push(Status::fail(
            "bad-fvar-instances",
            &format!(
                "fvar instances are incorrect:\n\n{}\n\n{}\n\n",
                hints.join("\n"),
                md_table.build().with(tabled::settings::Style::markdown())
            ),
        ));
    } else if same_names
        .into_iter()
        .any(|i| instances[*i] != expected_instances[*i])
    {
        problems.push(Status::warn(
            "suspicious-fvar-coords",
            &format!(
                "fvar instance coordinates for non-wght axes are not the same as the fvar defaults. This may be intentional so please check with the font author:\n\n{}\n\n",
                md_table.build().with(tabled::settings::Style::markdown())
            )
        ));
    }
    return_result(problems)
}

fn fix_fvar_instances(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    if !f.is_variable_font() {
        return Ok(false);
    }
    let new_binary =
        build_fvar_instances(f.font(), None).map_err(|e| FontspectorError::Fix(e.to_string()))?;
    t.set(new_binary);
    Ok(true)
}

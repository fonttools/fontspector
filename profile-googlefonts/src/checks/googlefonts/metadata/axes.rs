use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, FileTypeConvert};
use google_fonts_axisregistry::AxisRegistry;
use hashbrown::HashSet;

use crate::checks::googlefonts::metadata::family_proto;

#[check(
    id = "googlefonts/metadata/axes",
    rationale = "
        
        Each axis range in a METADATA.pb file must be registered, and within the bounds
        of the axis definition in the Google Fonts Axis Registry, available at
        https://github.com/google/fonts/tree/main/axisregistry
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3010 and https://github.com/fonttools/fontbakery/issues/3022",
    title = "Validate METADATA.pb axes values.",
    implementation = "all"
)]
fn axes(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let msg = family_proto(mdpb)?;
    // Skip if no variable fonts
    let fonts = msg
        .fonts
        .iter()
        .flat_map(|f| f.filename.as_ref())
        .flat_map(|f| c.get_file(f))
        .flat_map(|f| TTF.from_testable(f))
        .collect::<Vec<_>>();
    if !fonts.iter().any(|f| f.is_variable_font()) {
        skip!("no-variable", "No variable fonts found in the family");
    }

    let axisregistry = AxisRegistry::new();
    for axis in msg.axes.iter() {
        if let Some(registry_tag) = axisregistry.get(axis.tag()) {
            // Check bounds are correct, googlefonts/metadata/axisregistry_bounds
            if axis.min_value() < registry_tag.min_value()
                || axis.max_value() > registry_tag.max_value()
            {
                problems.push(Status::fail(
                    "bad-axis-range",
                    &format!(
                        "The range in the font variation axis '{}' ({}) min:{} max:{} does not comply with the expected maximum range, as defined on Google Fonts Axis Registry (min:{} max:{}).",
                        axis.tag(),
                        registry_tag.display_name(),
                        axis.min_value(),
                        axis.max_value(),
                        registry_tag.min_value(),
                        registry_tag.max_value()
                    ),
                ));
            }
        } else {
            // googlefonts/metadata/axisregistry_valid_tags, as was.
            problems.push(Status::fail(
                "bad-axis-tag",
                &format!(
                    "The font variation axis '{}' is not yet registered in the Google Fonts Axis Registry",
                    axis.tag()
                ),
            ));
        }
    }

    // googlefonts/metadata/consistent_axis_enumeration

    // Let's get the set of axes in the fonts
    let font_axes = fonts
        .iter()
        .flat_map(|f| f.font().axes().iter())
        .map(|ax| ax.tag().to_string())
        .collect::<HashSet<_>>();
    let md_axes = msg
        .axes
        .iter()
        .map(|ax| ax.tag().to_string())
        .collect::<HashSet<_>>();
    let missing = font_axes.difference(&md_axes).collect::<Vec<_>>();
    let extra = md_axes.difference(&font_axes).collect::<Vec<_>>();

    if !missing.is_empty() {
        problems.push(Status::fail(
            "missing-axes",
            &format!(
                "The font variation axes:\n\n{}\nare present in the font's fvar table but are not declared on the METADATA.pb file.",
                bullet_list(context, &missing)
            ),
        ));
    }
    if !extra.is_empty() {
        problems.push(Status::fail(
            "extra-axes",
            &format!(
                "The METADATA.pb file lists font variation axes that are not supported by this family:\n\n{}",
                bullet_list(context, &extra)
            ),
        ));
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use std::collections::HashMap;

    use super::axes;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        fontspector_checkapi::codetesting::run_check_with_config(
            axes,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    fn modified_metadata(old: &str, new: &str) -> Testable {
        let original = test_able("cabinvf/METADATA.pb");
        let text = String::from_utf8(original.contents).unwrap();
        Testable::new_with_contents("METADATA.pb", text.replacen(old, new, 1).into_bytes())
    }

    #[test]
    fn test_check_metadata_axes() {
        let font = test_able("cabinvf/Cabin[wdth,wght].ttf");
        let mdpb = test_able("cabinvf/METADATA.pb");

        // Good: cabinvf axes are within registry bounds
        assert_pass(&run(vec![font.clone(), mdpb]));

        // Bad axis range: wdth min_value below registry minimum
        let bad_range = modified_metadata("min_value: 75.0", "min_value: 20.0");
        assert_results_contain(
            &run(vec![font.clone(), bad_range]),
            StatusCode::Fail,
            Some("bad-axis-range".to_string()),
        );

        // Bad axis tag: unregistered tag
        let bad_tag = modified_metadata("tag: \"wdth\"", "tag: \"crap\"");
        assert_results_contain(
            &run(vec![font.clone(), bad_tag]),
            StatusCode::Fail,
            Some("bad-axis-tag".to_string()),
        );

        // Missing axis: replace second axis tag so wght is absent from metadata
        let missing = modified_metadata("tag: \"wght\"", "tag: \"wdth\"");
        assert_results_contain(
            &run(vec![font.clone(), missing]),
            StatusCode::Fail,
            Some("missing-axes".to_string()),
        );

        // Extra axis: add an axis that the font doesn't have
        let extra = modified_metadata("tag: \"wght\"", "tag: \"ouch\"");
        assert_results_contain(
            &run(vec![font, extra]),
            StatusCode::Fail,
            Some("extra-axes".to_string()),
        );
    }
}

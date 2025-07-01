use fontspector_checkapi::{prelude::*, skip};

use crate::{checks::googlefonts::metadata::family_proto, network_conditions::production_metadata};

#[check(
    id = "googlefonts/metadata/includes_production_subsets",
    rationale = "
        
        Check METADATA.pb file includes the same subsets as the family in production.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2989",
    title = "Check METADATA.pb includes production subsets.",
    applies_to = "MDPB"
)]
fn includes_production_subsets(c: &Testable, context: &Context) -> CheckFnResult {
    skip!(
        context.skip_network,
        "network-check",
        "Skipping network check"
    );
    let msg = family_proto(c)?;
    let production_metadata = production_metadata(context).map_err(|e| {
        FontspectorError::General(format!("Failed to fetch production metadata: {e:?}"))
    })?;
    let prod_subsets = production_metadata
        .get("familyMetadataList")
        .ok_or_else(|| {
            FontspectorError::General(
                "Failed to get familyMetadataList from production metadata".to_string(),
            )
        })?
        .as_array()
        .ok_or_else(|| FontspectorError::General("familyMetadataList is not an array".to_string()))?
        .iter()
        .find(|i| i.get("family").and_then(|f| f.as_str()) == Some(msg.name()))
        .and_then(|i| i.get("subsets"))
        .and_then(|s| s.as_array())
        .map(|s| {
            s.iter()
                .map(|i| i.as_str().unwrap_or_default())
                .collect::<Vec<&str>>()
        })
        .unwrap_or_default();
    let local_subsets = msg.subsets;
    let missing_subsets = prod_subsets
        .iter()
        .filter(|s| !local_subsets.contains(&s.to_string()))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    Ok(if missing_subsets.is_empty() {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "missing-subsets",
            &format!(
                "The following subsets are missing: {}",
                missing_subsets.join(", ")
            ),
        )
    })
}

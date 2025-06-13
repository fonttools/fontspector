#![deny(clippy::unwrap_used, clippy::expect_used)]
mod checks;
use serde_json::json;
use std::collections::HashMap;

use fontspector_checkapi::{ProfileBuilder, Registry};

pub struct Fontwerk;
impl fontspector_checkapi::Plugin for Fontwerk {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        let builder = ProfileBuilder::new()
            .include_profile("googlefonts")
            // exclude googlefonts checks
            .exclude_check("googlefonts/canonical_filename")
            .exclude_check("googlefonts/family/italics_have_roman_counterparts")  // May need some improvements before we decide to include this one.
            .exclude_check("googlefonts/font_copyright")
            .exclude_check("googlefonts/fstype")
            .exclude_check("googlefonts/gasp")
            .exclude_check("googlefonts/metadata/includes_production_subsets")
            .exclude_check("googlefonts/meta/script_lang_tags")
            .exclude_check("googlefonts/name/description_max_length")
            .exclude_check("googlefonts/name/line_breaks")
            .exclude_check("googlefonts/production_glyphs_similarity")
            .exclude_check("googlefonts/vendor_id") // Custom fontwerk test below
            .exclude_check("googlefonts/version_bump")
            .exclude_check("fontdata_namecheck")
            .include_profile("opentype")
            .add_section("Fontwerk Checks")
            .add_and_register_check(checks::fontwerk::name_entries)
            //.add_and_register_check(checks::fontwerk::vendor_id)
            // TODO: implement other Fontwerk checks
            // .add_and_register_check("fontwerk/names_match_default_fvar")
            // .add_and_register_check("fontwerk/style_linking");
            .with_configuration_defaults(
                "opentype/vendor_id",
                HashMap::from([
                    ("vendor_id".to_string(), json!("WERK"))
                ]),
            )
            .with_configuration_defaults(
                "fontwerk/name_entries",
                HashMap::from([
                    ("MANUFACTURER".to_string(), json!("Fontwerk")),
                    ("VENDOR_URL".to_string(), json!("https://fontwerk.com")),
                    ("LICENSE_DESCRIPTION".to_string(), json!("This Font Software is the property of Fontwerk GmbH its use by you is covered under the terms of an End-User License Agreement (EULA). Unless you have entered into a specific license agreement granting you additional rights, your use of this Font Software is limited by the terms of the actual license agreement you have entered into with Fontwerk. If you have any questions concerning your rights you should review the EULA you received with the software or contact Fontwerk. A copy of the EULA for this Font Software can be found on https://fontwerk.com/licensing.")),
                    ("LICENSE_URL".to_string(), json!("https://fontwerk.com")),
                    ]),
            );
        builder.build("fontwerk", cr)
    }
}

#[cfg(not(target_family = "wasm"))]
pluginator::plugin_implementation!(fontspector_checkapi::Plugin, Fontwerk);
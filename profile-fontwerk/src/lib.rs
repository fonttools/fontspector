#![deny(clippy::unwrap_used, clippy::expect_used)]
mod checks;
use serde_json::json;
use std::collections::HashMap;

use fontspector_checkapi::{Override, ProfileBuilder, Registry, StatusCode};

pub struct Fontwerk;
impl fontspector_checkapi::Plugin for Fontwerk {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        let builder = ProfileBuilder::new()
            .include_profile("googlefonts")
            .with_overrides("valid_glyphnames", vec![
                Override::new("found-invalid-names", StatusCode::Warn, "")
            ])
            .with_overrides("soft_hyphen", vec![
                Override::new("softhyphen", StatusCode::Fail, "For Fontwerk, the 'Soft Hyphen' character must be removed.")
            ])
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
            .exclude_check("googlefonts/font_names")
            .exclude_check("googlefonts/varfont/has_HVAR")
            .exclude_check("googlefonts/weightclass")
            .exclude_check("control_chars")
            .exclude_check("fontdata_namecheck")
            .include_profile("opentype")
            .add_section("Fontwerk Checks")
            .add_and_register_check(checks::fontwerk::name_entries)
            .add_and_register_check(checks::fontwerk::name_consistency)
            .add_and_register_check(checks::fontwerk::fstype)
            .add_and_register_check(checks::fontwerk::glyph_coverage)
            .add_and_register_check(checks::fontwerk::weightclass)
            // TODO: implement other Fontwerk checks
            // .add_and_register_check("fontwerk/names_match_default_fvar")
            .include_profile("universal")
            .with_configuration_defaults(
                "required_name_ids",
                HashMap::from([
                    ("required_name_ids".to_string(), json!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 25])),
                ]),
            )
            .with_configuration_defaults(
                "opentype/vendor_id",
                HashMap::from([
                    ("vendor_id".to_string(), json!("WERK"))
                ]),
            )
            .with_configuration_defaults(
                "fontwerk/name_entries",
                HashMap::from([
                    ("COPYRIGHT_NOTICE".to_string(), json!(r"regex:Copyright \(c\) (\d{4}(-\d{4})?, )*\d{4}(-\d{4})? Fontwerk GmbH\. All rights reserved\.")),
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

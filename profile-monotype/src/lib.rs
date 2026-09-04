#![deny(clippy::unwrap_used, clippy::expect_used)]
mod checks;
use serde_json::json;
use std::collections::HashMap;

use fontspector_checkapi::{FontspectorError, Override, ProfileBuilder, Registry, StatusCode};

pub struct Monotype;
impl fontspector_checkapi::ProfileProvider for Monotype {
    fn register(&self, cr: &mut Registry) -> Result<(), FontspectorError> {
        let builder = ProfileBuilder::new()
            .include_profile("googlefonts")
            .with_overrides("valid_glyphnames", vec![
                Override::new("found-invalid-names", StatusCode::Warn, "")
            ])
            // exclude googlefonts checks
            .exclude_check("googlefonts/canonical_filename")
            // .exclude_check("googlefonts/family/italics_have_roman_counterparts")
            .exclude_check("googlefonts/font_copyright")
            .exclude_check("googlefonts/fstype")
            .exclude_check("googlefonts/metadata/includes_production_subsets")
            .exclude_check("googlefonts/meta/script_lang_tags")
            .exclude_check("googlefonts/name/description_max_length")
            .exclude_check("googlefonts/name/line_breaks")
            .exclude_check("googlefonts/production_glyphs_similarity")
            .exclude_check("googlefonts/vendor_id") // Custom monotype test below
            .exclude_check("googlefonts/version_bump")
            .exclude_check("googlefonts/font_names")
            .exclude_check("googlefonts/varfont/has_HVAR")
            .exclude_check("googlefonts/weightclass")
            .exclude_check("control_chars")
            .exclude_check("fontdata_namecheck")
            .include_profile("opentype")
            .add_section("Monotype Checks")
            .add_and_register_check(checks::monotype::fstype)
            .with_configuration_defaults(
                "monotype/fstype",
                HashMap::from([
                    ("fstype_value".to_string(), json!(4))
                ]),
            )
            // TODO: implement more Monotype-specific checks
            .include_profile("universal")
            .with_configuration_defaults(
                "universal/required_name_ids",
                HashMap::from([
                    ("required_name_ids".to_string(), json!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 25])),
                ]),
            )
            .with_configuration_defaults(
                "opentype/vendor_id",
                HashMap::from([
                    ("vendor_id".to_string(), json!("MONO"))
                ]),
            )
            .include_profile("fontwerk")
            .exclude_check("fontwerk/glyph_coverage")
            .with_configuration_defaults(
                "fontwerk/name_entries",
                HashMap::from([
                    ("COPYRIGHT_NOTICE".to_string(), json!(r"regex:Copyright \(c\) (\d{4}(-\d{4})?, )*\d{4}(-\d{4})? Monotype Imaging Inc\. All rights reserved\.")),
                    ("MANUFACTURER".to_string(), json!("Monotype")),
                    ("VENDOR_URL".to_string(), json!("https://monotype.com")),
                    ("LICENSE_DESCRIPTION".to_string(), json!("This font software is the property of Monotype Imaging Inc., or one of its affiliated entities (collectively, Monotype) and its use by you is covered under the terms of a license agreement. You have obtained this font software either directly from Monotype or together with software distributed by one of the licensees of Monotype. This software is a valuable asset of Monotype. Unless you have entered into a specific license agreement granting you additional rights, your use of this software is limited by the terms of the actual license agreement you have entered into with Monotype. You may not copy or distribute this software. If you have any questions concerning your rights you should review the license agreement you received with the software. You can learn more about Monotype by clicking here: www.monotype.com.")),
                    ("LICENSE_URL".to_string(), json!("https://monotype.com")),
                    ]),
            );
        builder.build("monotype", cr)
    }
}

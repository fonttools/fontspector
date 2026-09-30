#![deny(clippy::unwrap_used, clippy::expect_used)]
mod checks;
use serde_json::json;
use std::collections::HashMap;

use fontspector_checkapi::{FontspectorError, Override, ProfileBuilder, Registry, StatusCode};
use profile_fontwerk::checks as fw_checks;
use profile_googlefonts::checks as gf_checks;

pub struct Monotype;
impl fontspector_checkapi::ProfileProvider for Monotype {
    fn register(&self, cr: &mut Registry) -> Result<(), FontspectorError> {
        let builder = ProfileBuilder::new()
            .add_section("Selected Outline Checks")
            .add_and_register_check(gf_checks::outline::direction)
            .add_and_register_check(gf_checks::outline::jaggy_segments)
            .add_and_register_check(gf_checks::outline::colinear_vectors)
            .add_and_register_check(gf_checks::outline::short_segments)
            .add_and_register_check(gf_checks::outline::semi_vertical)
            .add_and_register_check(gf_checks::outline::alignment_miss)
            .add_and_register_check(gf_checks::outline::overlapping_path_segments)
            .add_section("Selected Family Checks")
            .add_and_register_check(gf_checks::googlefonts::family::tnum_horizontal_metrics)
            .add_and_register_check(gf_checks::googlefonts::family::equal_codepoint_coverage)
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
            .add_and_register_check(checks::monotype::vertical_metrics_sane)
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
            .with_overrides("valid_glyphnames", vec![
                Override::new("found-invalid-names", StatusCode::Warn, "")
            ])
            .add_section("Selected Name Table Checks") 
            .add_and_register_check(fw_checks::fontwerk::name_entries)
            .add_and_register_check(fw_checks::fontwerk::name_consistency)
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

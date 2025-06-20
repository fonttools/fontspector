mod checks;

use fontspector_checkapi::{ProfileBuilder, Registry};

pub struct Microsoft;
impl fontspector_checkapi::Plugin for Microsoft {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        let builder = ProfileBuilder::new()
            .include_profile("universal")
            .add_section("Metadata Checks")
            .add_and_register_check(checks::microsoft::copyright)
            .add_and_register_check(checks::microsoft::fstype)
            .add_and_register_check(checks::microsoft::license_description)
            .add_and_register_check(checks::microsoft::manufacturer)
            .add_and_register_check(checks::microsoft::trademark)
            .add_and_register_check(checks::microsoft::vendor_url)
            .add_and_register_check(checks::microsoft::version)
            .add_section("Name Checks")
            .add_and_register_check(checks::microsoft::office_ribz_req)
            .add_and_register_check(checks::name_id_1)
            .add_and_register_check(checks::name_id_2)
            .add_and_register_check(checks::name_length_req)
            .add_section("Metrics Checks")
            .add_and_register_check(checks::microsoft::vertical_metrics)
            .add_section("Variable Fonts Checks")
            .add_and_register_check(checks::microsoft::fvar_STAT_axis_ranges)
            .add_and_register_check(checks::microsoft::STAT_axis_values)
            .add_and_register_check(checks::microsoft::STAT_table_axis_order)
            .add_and_register_check(checks::microsoft::STAT_table_eliding_bit)
            .add_section("Glyph Checks")
            .add_and_register_check(checks::tnum_glyphs_equal_widths)
            .exclude_check("fontbakery_version")
            .exclude_check("STAT_in_statics")
            .exclude_check("tabular_kerning");
        builder.build("microsoft", cr)
    }
}

#[cfg(not(target_family = "wasm"))]
pluginator::plugin_implementation!(fontspector_checkapi::Plugin, Microsoft);

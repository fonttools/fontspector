//! The ISO15008 profile for Fontspector
//!
//!  This profile implements checks for the ISO 15008 standard, which is related to
//!  the design of fonts for use in automotive displays.
mod checks;

use fontspector_checkapi::{ProfileBuilder, Registry};

/// This is the main plugin struct for the ISO15008 profile.
pub struct Iso15008;
impl fontspector_checkapi::Plugin for Iso15008 {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        let builder = ProfileBuilder::new()
            .add_section("Iso15008 Fonts Checks")
            .add_and_register_check(checks::iso15008::intercharacter_spacing)
            .add_and_register_check(checks::iso15008::interline_spacing)
            .add_and_register_check(checks::iso15008::interword_spacing)
            .add_and_register_check(checks::iso15008::proportions)
            .add_and_register_check(checks::iso15008::stem_width);
        builder.build("iso15008", cr)
    }
}

#![deny(clippy::unwrap_used, clippy::expect_used)]
mod checks;

use fontspector_checkapi::{Override, ProfileBuilder, Registry, StatusCode};

pub struct Fontwerk;
impl fontspector_checkapi::Plugin for Fontwerk {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        let builder = ProfileBuilder::new()
            .add_section("Fontwerk Fonts Checks")
            // "Fontwerk Fonts Checks" = ["fontwerk/vendor_id"]
            .add_and_register_check(checks::fontwerk::vendor_id)
            .include_profile("universal");
        builder.build("fontwerk", cr)
    }
}

#[cfg(not(target_family = "wasm"))]
pluginator::plugin_implementation!(fontspector_checkapi::Plugin, Fontwerk);

mod checks;

use fontspector_checkapi::{FontspectorError, ProfileBuilder, Registry};

pub struct SILfonts;
impl fontspector_checkapi::ProfileProvider for SILfonts {
    fn register(&self, cr: &mut Registry) -> Result<(), FontspectorError> {
        let builder = ProfileBuilder::new()
            .add_section("SIL Checks")
            .add_and_register_check(checks::silfonts::name::version_format);
        builder.build("silfonts", cr)
    }
}

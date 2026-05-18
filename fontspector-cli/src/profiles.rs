use fontspector_checkapi::{Profile, ProfileProvider, Registry};
use profile_fontwerk::Fontwerk;
use profile_googlefonts::GoogleFonts;
use profile_iso15008::Iso15008;
use profile_opentype::OpenType;
use profile_universal::Universal;
use std::{io::Read, path::PathBuf};

use crate::args::Args;

pub(crate) fn register_and_return_toml_profile(
    args: &Args,
    registry: &mut Registry<'static>,
) -> String {
    // Name should be path basename without extension
    let path = PathBuf::from(&args.profile);
    let name = path.file_stem().unwrap_or_default().to_string_lossy();
    match std::fs::File::open(&path) {
        Ok(mut file) => {
            log::info!("Loading profile from file {name:?}");
            let mut toml = String::new();
            if let Err(e) = file.read_to_string(&mut toml) {
                log::error!("Could not read profile {name:}: {e:}");
                std::process::exit(1);
            }
            let profile: Profile = Profile::from_toml(&toml).unwrap_or_else(|e| {
                log::error!("Could not parse profile {name:}: {e:}");
                std::process::exit(1);
            });

            registry
                .register_profile(&name, profile, true)
                .unwrap_or_else(|e| {
                    log::error!("Could not register profile {name:}: {e:}");
                    std::process::exit(1);
                });
        }
        Err(e) => {
            log::error!("Could not open profile file {:}: {:?}", args.profile, e);
            std::process::exit(1);
        }
    }
    name.to_string()
}

#[allow(unused_variables)]
pub(crate) fn register_core_profiles(args: &Args, registry: &mut Registry<'static>) {
    #[allow(clippy::expect_used)] // If this fails, I *want* to panic
    OpenType
        .register(registry)
        .expect("Couldn't register opentype profile, fontspector bug");
    #[allow(clippy::expect_used)] // If this fails, I *want* to panic
    Universal
        .register(registry)
        .expect("Couldn't register universal profile, fontspector bug");

    #[allow(clippy::expect_used)] // If this fails, I *want* to panic
    GoogleFonts
        .register(registry)
        .expect("Couldn't register googlefonts profile, fontspector bug");

    #[allow(clippy::expect_used)] // If this fails, I *want* to panic
    Iso15008
        .register(registry)
        .expect("Couldn't register iso15008 profile, fontspector bug");

    #[allow(clippy::expect_used)] // If this fails, I *want* to panic
    Fontwerk
        .register(registry)
        .expect("Couldn't register fontwerk profile, fontspector bug");
}

#[cfg(feature = "python")]
use fontbakery_bridge::FontbakeryBridge;

use fontspector_checkapi::{Plugin, Profile, Registry};
use profile_googlefonts::GoogleFonts;
use profile_iso15008::Iso15008;
use profile_opentype::OpenType;
use profile_universal::Universal;
use profile_fontwerk::Fontwerk;
use std::io::Read;
use std::path::PathBuf;

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
            log::info!("Loading profile from file {:?}", name);
            let mut toml = String::new();
            if let Err(e) = file.read_to_string(&mut toml) {
                log::error!("Could not read profile {:}: {:}", name, e);
                std::process::exit(1);
            }
            let profile: Profile = Profile::from_toml(&toml).unwrap_or_else(|e| {
                log::error!("Could not parse profile {:}: {:}", name, e);
                std::process::exit(1);
            });

            #[cfg(feature = "python")]
            if args.use_python {
                for python_file in profile.check_definitions.iter() {
                    if let Err(e) = load_python_profile(registry, python_file, &path) {
                        log::error!("Could not load python profile {:}: {:}", python_file, e);
                        std::process::exit(1);
                    }
                }
            }

            registry
                .register_profile(&name, profile)
                .unwrap_or_else(|e| {
                    log::error!("Could not register profile {:}: {:}", name, e);
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
    #[cfg(feature = "python")]
    if args.use_python {
        // Python implementations first, I want to override them
        #[allow(clippy::expect_used)] // If this fails, I *want* to panic
        FontbakeryBridge
            .register(registry)
            .expect("Couldn't register fontbakery bridge, fontspector bug");
    }

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

#[cfg(feature = "python")]
pub fn load_python_profile(
    registry: &mut Registry<'static>,
    python_file: &str,
    relative_to: &PathBuf,
) -> Result<(), String> {
    // If the file is relative, make it absolute
    let python_path = if !PathBuf::from(python_file).is_absolute() {
        std::fs::canonicalize(relative_to)
            .map_err(|e| e.to_string())?
            .parent()
            .ok_or("Could not get parent directory")?
            .join(python_file)
    } else {
        PathBuf::from(python_file)
    };
    log::info!("Loading python profile from file {:?}", python_path);
    let mut file = std::fs::File::open(&python_path).map_err(|e| {
        format!(
            "Could not open python profile file {:?}: {:?}",
            python_path, e
        )
    })?;
    let mut source = String::new();
    file.read_to_string(&mut source).map_err(|e| {
        format!(
            "Could not read python profile file {:?}: {:?}",
            python_path, e
        )
    })?;
    // Turn the path into a valid Python module name
    let module_name = python_file
        .replace("-", "_")
        .replace("\\", ".")
        .replace("/", ".")
        .replace(".py", "");
    log::debug!("Module name: {:?}", module_name);
    fontbakery_bridge::register_python_checks(&module_name, &source, registry)
}

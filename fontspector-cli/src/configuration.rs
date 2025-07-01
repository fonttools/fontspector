use std::collections::HashMap;

use fontspector_checkapi::{CheckId, Override};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::args::Args;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct UserConfigurationFile {
    #[serde(default)]
    pub exclude_checks: Option<Vec<String>>,
    #[serde(default)]
    pub explicit_checks: Option<Vec<String>>,
    #[serde(default)]
    pub overrides: Option<Vec<Override>>,

    #[serde(flatten)]
    pub per_check_config: HashMap<CheckId, Value>,
}

pub(crate) fn load_configuration(args: &Args) -> UserConfigurationFile {
    let Some(configfile) = args.configuration.as_ref() else {
        return UserConfigurationFile::default();
    };

    let contents = std::fs::read_to_string(configfile).unwrap_or_else(|e| {
        log::error!("Could not read configuration file {configfile}: {e:}");
        std::process::exit(1)
    });

    if configfile.ends_with(".toml") {
        return toml::from_str(&contents).unwrap_or_else(|e| {
            log::error!("Could not parse configuration file: {e:}");
            std::process::exit(1)
        });
    }

    if configfile.ends_with(".json") {
        return serde_json::from_str(&contents).unwrap_or_else(|e| {
            log::error!("Could not parse configuration file: {e:}");
            std::process::exit(1)
        });
    }

    log::error!("Configuration file must be in TOML or JSON format: {configfile}");
    std::process::exit(1);
}

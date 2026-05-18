use std::{
    collections::HashMap, path::PathBuf, process::Command as ProcessCommand, sync::LazyLock,
};

use crate::{prelude::*, CheckResult, ProfileProvider};
use clap::{Arg, ArgAction, Command};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Data about an external check provided by a subprocess plugin
#[derive(Clone)]
pub struct ExternalCheckData {
    /// Path to the plugin executable
    pub plugin_path: String,
    /// The check ID this data refers to
    pub check_id: String,
    /// Whether this check runs on a collection or single file
    pub runs_on_collection: bool,
}

/// Global registry of external checks, keyed by check ID
static EXTERNAL_CHECKS: LazyLock<std::sync::Mutex<HashMap<String, ExternalCheckData>>> =
    LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// Register an external check
pub fn register_external_check(check_id: String, data: ExternalCheckData) {
    if let Ok(mut registry) = EXTERNAL_CHECKS.lock() {
        registry.insert(check_id, data);
    }
}

/// The metadata about a plugin, which is passed to fontspector when the plugin is loaded
#[derive(Serialize, Deserialize)]
pub struct PluginMetadata {
    /// The version of the plugin API this plugin was built against. This is used to ensure compatibility between plugins and fontspector.
    pub api_version: u32,
    /// The name of the plugin
    pub plugin_name: String,
    /// The profiles provided by this plugin, mapped by name
    pub profiles: HashMap<String, Value>,
    /// The checks provided by this plugin
    pub checks: Vec<CheckMetadata>,
    /// The filetypes provided by this plugin, mapped by name to a glob pattern
    pub filetypes: HashMap<String, String>,
}

impl PluginMetadata {
    /// Construct new plugin metadata
    pub fn new(plugin_name: &str) -> Self {
        Self {
            api_version: 1,
            plugin_name: plugin_name.to_string(),
            profiles: HashMap::new(),
            checks: Vec::new(),
            filetypes: HashMap::new(),
        }
    }

    /// Register that this plugin provides a profile with the given name and definition
    pub fn register_profile(
        mut self,
        name: &str,
        profile: &Profile,
    ) -> Result<Self, FontspectorError> {
        self.profiles
            .insert(name.to_string(), serde_json::to_value(profile)?);
        Ok(self)
    }

    /// Register that this plugin provides a check with the given definition
    pub fn add_check(mut self, check: Check) -> Self {
        self.checks.push(CheckMetadata::from(check));
        self
    }

    /// Register that this plugin provides a filetype with the given name and pattern
    pub fn register_filetype(mut self, name: &str, pattern: &str) -> Self {
        self.filetypes.insert(name.to_string(), pattern.to_string());
        self
    }
}

#[derive(Serialize, Deserialize)]
/// Metadata about a check, extracted from the check definition and passed to fontspector when the plugin is loaded
pub struct CheckMetadata {
    /// The check's unique identifier
    pub id: String,
    /// Title to be displayed to the user
    pub title: String,
    /// A short description of the check
    pub rationale: String,
    /// URL where the check was proposed
    pub proposal: Vec<String>,
    /// The filetype this check applies to
    pub applies_to: String,
    /// Does this check run on a collection of files?
    pub runs_on_collection: bool,
    /// Metadata for the check in JSON format
    pub metadata: serde_json::Value,
    /// Whether or not a hotfix is available for this check
    pub hotfix_available: bool,
    /// Whether or not a source fix is available for this check
    pub sourcefix_available: bool,
}

impl From<Check<'_>> for CheckMetadata {
    fn from(check: Check) -> Self {
        Self {
            id: check.id.to_string(),
            title: check.title.to_string(),
            rationale: check.rationale.to_string(),
            proposal: check.proposal.iter().map(|s| s.to_string()).collect(),
            applies_to: check.applies_to.to_string(),
            runs_on_collection: check.runs_on_collection(),
            metadata: check.metadata(),
            hotfix_available: check.hotfix.is_some(),
            sourcefix_available: check.fix_source.is_some(),
        }
    }
}
/// Build a default context for a check, which can be used when running a check from the plugin without any user configuration
fn default_context(check: &Check<'_>) -> Context {
    Context {
        skip_network: false,
        network_timeout: None,
        configuration: Default::default(),
        check_metadata: check.metadata(),
        full_lists: false,
        cache: Default::default(),
        overrides: vec![],
        check_id: None,
    }
}

/// Run a single check by id on a list of files, returning the result
fn run_single_check(
    registry: &Registry<'static>,
    check_id: &str,
    files: &[String],
) -> Result<CheckResult, String> {
    let check = registry
        .checks
        .get(check_id)
        .ok_or_else(|| format!("Unknown check id: {check_id}"))?;

    if check.runs_on_collection() {
        let filenames: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
        let collection = TestableCollection::from_filenames(&filenames, None)
            .map_err(|e| format!("Could not build testable collection: {e}"))?;
        let testable = TestableType::Collection(&collection);
        check
            .run(&testable, &default_context(check), Some("test"))
            .ok_or_else(|| "Check did not run for this collection".to_string())
    } else {
        if files.len() != 1 {
            return Err("Single-file checks expect exactly one file argument".to_string());
        }
        #[allow(clippy::indexing_slicing)] // We just checked that files has exactly one element
        let testable =
            Testable::new(&files[0]).map_err(|e| format!("Could not open testable file: {e}"))?;
        let testable = TestableType::Single(&testable);
        check
            .run(&testable, &default_context(check), Some("test"))
            .ok_or_else(|| "Check did not run for this file".to_string())
    }
}

/// Build the plugin metadata from the registry
fn build_metadata(
    registry: &Registry,
    pluginname: &str,
) -> Result<PluginMetadata, FontspectorError> {
    let mut metadata = PluginMetadata::new(pluginname);
    for (name, profile) in &registry.profiles {
        metadata = metadata.register_profile(name, profile)?;
    }
    for check in registry.checks.values() {
        metadata = metadata.add_check(check.clone());
    }
    for (name, filetype) in &registry.filetypes {
        metadata = metadata.register_filetype(name, filetype.pattern);
    }
    Ok(metadata)
}

/// Are we a fontspector plugin?
pub(crate) fn current_executable_is_plugin() -> bool {
    std::env::var("FONTSPECTOR_PLUGIN").is_ok()
}

/// The main entry point for a plugin, which sets up the registry and runs the appropriate check based on the command line arguments
pub fn plugin_main<F: ProfileProvider>(profile_provider: F) {
    std::env::set_var("FONTSPECTOR_PLUGIN", "1");
    let mut registry = Registry::new();
    profile_provider
        .register(&mut registry)
        .unwrap_or_else(|e| {
            eprintln!("Error setting up plugin: {e}");
            std::process::exit(1);
        });

    // This isn't the executable name, but humans shouldn't really be using this anyway.
    let cmd = Command::new("fontspector-plugin")
        .about("Fontspector plugin")
        .version("1.0")
        .arg(
            Arg::new("metadata")
                .long("metadata")
                .help("Query plugin for available profiles, checks, and filetypes in JSON format")
                .action(ArgAction::SetTrue)
                .conflicts_with("check_id"),
        )
        .arg(
            Arg::new("check_id")
                .long("check")
                .help("Execute a single check on one or more files")
                .value_name("CHECK_ID")
                .conflicts_with("metadata"),
        )
        .arg(
            Arg::new("files")
                .help("Files to check (single file for single-file checks, multiple for collection checks)")
                .required(false)
                .num_args(1..)
                .requires("check_id")
                .value_name("FILE"),
        )
        .subcommand(
            Command::new("metadata")
                .about("Query plugin for available profiles, checks, and filetypes in JSON format")
        )
        .subcommand(
            Command::new("check")
                .about("Execute a single check on one or more files")
                .arg(
                    Arg::new("check_id")
                        .help("The check ID to execute")
                        .required(true)
                        .value_name("CHECK_ID")
                )
                .arg(
                    Arg::new("files")
                        .help("Files to check (single file for single-file checks, multiple for collection checks)")
                        .required(true)
                        .num_args(1..)
                        .value_name("FILE")
                )
        );

    let matches = cmd.get_matches();

    let metadata_requested = matches.get_flag("metadata");
    let check_id_from_flag = matches.get_one::<String>("check_id");

    match matches.subcommand() {
        Some(("metadata", _)) | None if metadata_requested => {
            let metadata = build_metadata(&registry, "plugin").unwrap_or_else(|e| {
                eprintln!("Error: Could not build metadata: {e}");
                std::process::exit(1);
            });
            match serde_json::to_string_pretty(&metadata) {
                Ok(json) => println!("{json}"),
                Err(e) => {
                    eprintln!("Error: Could not serialize metadata: {e}");
                    std::process::exit(1);
                }
            }
        }
        Some(("check", matches)) => {
            #[allow(clippy::expect_used)] // clap enforces required arguments
            let check_id = matches
                .get_one::<String>("check_id")
                .expect("check_id is required")
                .as_str();
            #[allow(clippy::expect_used)] // clap enforces required arguments
            let files: Vec<String> = matches
                .get_many::<String>("files")
                .expect("files is required")
                .cloned()
                .collect();
            match run_single_check(&registry, check_id, &files) {
                Ok(result) => match serde_json::to_string_pretty(&result) {
                    Ok(json) => println!("{json}"),
                    Err(e) => {
                        eprintln!("Error: Could not serialize check result: {e}");
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        None if check_id_from_flag.is_some() => {
            #[allow(clippy::expect_used)] // clap enforces required arguments
            let check_id = check_id_from_flag.expect("check_id is required").as_str();
            #[allow(clippy::expect_used)] // clap enforces required arguments
            let files: Vec<String> = matches
                .get_many::<String>("files")
                .expect("files are required when --check is used")
                .cloned()
                .collect();
            match run_single_check(&registry, check_id, &files) {
                Ok(result) => match serde_json::to_string_pretty(&result) {
                    Ok(json) => println!("{json}"),
                    Err(e) => {
                        eprintln!("Error: Could not serialize check result: {e}");
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!(
                "No plugin command provided. Use --metadata or --check <CHECK_ID> <FILE>... ."
            );
            std::process::exit(2);
        }
    }
}

/// Query plugin metadata using the plugin protocol
fn query_plugin_metadata(plugin_path: &str) -> Result<std::process::Output, String> {
    let output = ProcessCommand::new(plugin_path)
        .arg("--metadata")
        .output()
        .map_err(|e| format!("Failed to spawn plugin subprocess: {e}"))?;

    if output.status.success() {
        return Ok(output);
    }

    let fallback_output = ProcessCommand::new(plugin_path)
        .arg("metadata")
        .output()
        .map_err(|e| format!("Failed to spawn plugin subprocess: {e}"))?;

    if fallback_output.status.success() {
        Ok(fallback_output)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let fallback_stderr = String::from_utf8_lossy(&fallback_output.stderr);
        Err(format!(
            "Plugin metadata query failed: {stderr}{sep}{fallback_stderr}",
            sep = if fallback_stderr.is_empty() {
                ""
            } else {
                "\nFallback protocol stderr: "
            }
        ))
    }
}

/// Run a plugin check using the plugin protocol
fn run_plugin_check(
    plugin_path: &str,
    check_id: &str,
    files: &[String],
) -> Result<std::process::Output, String> {
    let output = ProcessCommand::new(plugin_path)
        .arg("--check")
        .arg(check_id)
        .args(files)
        .output()
        .map_err(|e| format!("Failed to spawn plugin subprocess: {e}"))?;

    if output.status.success() {
        return Ok(output);
    }

    let fallback_output = ProcessCommand::new(plugin_path)
        .arg("check")
        .arg(check_id)
        .args(files)
        .output()
        .map_err(|e| format!("Failed to spawn plugin subprocess: {e}"))?;

    if fallback_output.status.success() {
        Ok(fallback_output)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let fallback_stderr = String::from_utf8_lossy(&fallback_output.stderr);
        Err(format!(
            "Plugin subprocess failed: {stderr}{sep}{fallback_stderr}",
            sep = if fallback_stderr.is_empty() {
                ""
            } else {
                "\nFallback protocol stderr: "
            }
        ))
    }
}

/// Run an external check in a subprocess
fn run_external_check(
    plugin_path: &str,
    check_id: &str,
    files: &[String],
) -> Result<CheckResult, String> {
    let output = run_plugin_check(plugin_path, check_id, files)?;

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| format!("Plugin subprocess returned invalid UTF-8: {e}"))?;

    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse plugin response: {e}"))
}

/// Wrapper function for CheckOne implementation that delegates to an external plugin
fn external_check_one(testable: &Testable, context: &Context) -> CheckFnResult {
    let check_id = context.check_id.as_ref().ok_or(FontspectorError::General(
        "External check called without check_id in context".to_string(),
    ))?;

    let external_checks = EXTERNAL_CHECKS.lock().map_err(|_| {
        FontspectorError::General("Failed to lock external checks registry".to_string())
    })?;

    let external_data = external_checks
        .get(check_id)
        .ok_or(FontspectorError::General(format!(
            "Unknown external check: {check_id}"
        )))?;

    let files = vec![testable.filename.to_string_lossy().to_string()];
    let result = run_external_check(&external_data.plugin_path, check_id, &files)
        .map_err(FontspectorError::General)?;

    Ok(Box::new(result.subresults.into_iter()))
}

/// Wrapper function for CheckAll implementation that delegates to an external plugin
fn external_check_all(collection: &TestableCollection, context: &Context) -> CheckFnResult {
    let check_id = context.check_id.as_ref().ok_or(FontspectorError::General(
        "External check called without check_id in context".to_string(),
    ))?;

    let external_checks = EXTERNAL_CHECKS.lock().map_err(|_| {
        FontspectorError::General("Failed to lock external checks registry".to_string())
    })?;

    let external_data = external_checks
        .get(check_id)
        .ok_or(FontspectorError::General(format!(
            "Unknown external check: {check_id}"
        )))?;

    let files: Vec<String> = collection
        .iter()
        .map(|t| t.filename.to_string_lossy().to_string())
        .collect();

    let result = run_external_check(&external_data.plugin_path, check_id, &files)
        .map_err(FontspectorError::General)?;

    Ok(Box::new(result.subresults.into_iter()))
}

/// Load an external plugin from a path and register its checks/profiles/filetypes
pub fn load_external_plugin(
    plugin_path: &str,
    registry: &mut Registry<'static>,
) -> Result<(), String> {
    let output = query_plugin_metadata(plugin_path)?;

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| format!("Plugin subprocess returned invalid UTF-8: {e}"))?;

    let metadata: PluginMetadata = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse plugin metadata: {e}"))?;

    // Register filetypes
    for (name, pattern) in &metadata.filetypes {
        let pattern_static = Box::leak(Box::new(pattern.clone())) as &'static str;
        registry.register_filetype(name, FileType::new(pattern_static));
    }

    // Register profiles (stored as Value in metadata, need to convert)
    for (name, profile_value) in &metadata.profiles {
        let profile: Profile = serde_json::from_value(profile_value.clone())
            .map_err(|e| format!("Failed to parse profile {name}: {e}"))?;
        registry
            .register_profile(name, profile, true)
            .map_err(|e| format!("Failed to register profile {name}: {e}"))?;
    }

    // Register checks
    for check_meta in &metadata.checks {
        let check_impl = if check_meta.runs_on_collection {
            CheckImplementation::CheckAll(&external_check_all)
        } else {
            CheckImplementation::CheckOne(&external_check_one)
        };

        let check = Check {
            id: Box::leak(Box::new(check_meta.id.clone())),
            title: Box::leak(Box::new(check_meta.title.clone())),
            rationale: Box::leak(Box::new(check_meta.rationale.clone())),
            proposal: Box::leak(
                check_meta
                    .proposal
                    .iter()
                    .map(|s| Box::leak(Box::new(s.clone())) as &'static str)
                    .collect::<Box<[&'static str]>>(),
            ),
            implementation: check_impl,
            hotfix: None,
            fix_source: None,
            applies_to: Box::leak(Box::new(check_meta.applies_to.clone())),
            flags: CheckFlags::default(),
            _metadata: None,
        };

        // Register the external check data so the wrapper can find it
        register_external_check(
            check_meta.id.clone(),
            ExternalCheckData {
                plugin_path: plugin_path.to_string(),
                check_id: check_meta.id.clone(),
                runs_on_collection: check_meta.runs_on_collection,
            },
        );

        registry.register_check(check);
    }

    Ok(())
}

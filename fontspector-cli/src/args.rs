use std::collections::HashMap;

use crate::build;
use clap::{ArgAction, Parser};
use fontspector_checkapi::StatusCode;

/// Quality control for OpenType fonts
#[derive(Parser, Debug)]
#[clap(author, version, long_version = build::CLAP_LONG_VERSION, about, long_about = None)]
pub struct Args {
    /// Plugins to load
    #[clap(long, value_delimiter = ',')]
    pub plugins: Vec<String>,

    /// Profile to check
    #[clap(short, long, default_value = "universal")]
    pub profile: String,

    /// List the checks available in the selected profile
    #[clap(short = 'L', long)]
    pub list_checks: bool,

    #[clap(long)]
    pub list_checks_json: bool,

    #[cfg(not(debug_assertions))]
    /// Number of worker processes. Defaults to the number of logical CPUs.
    #[clap(short = 'J', long)]
    pub jobs: Option<usize>,

    /// Read configuration file (TOML/JSON)
    #[clap(long)]
    pub configuration: Option<String>,

    /// Explicit check-ids (or parts of their name) to be executed
    #[clap(short, long)]
    pub checkid: Option<Vec<String>>,

    /// Exclude check-ids (or parts of their name) from execution
    #[clap(short = 'x', long)]
    pub exclude_checkid: Option<Vec<String>>,

    /// Report full lists of items instead of abbreviated lists
    #[clap(long)]
    pub full_lists: bool,

    /// Threshold for emitting process error code 1
    #[clap(short, long, value_enum, default_value_t=StatusCode::Fail)]
    pub error_code_on: StatusCode,

    /// Increase logging
    #[clap(short, long, action = ArgAction::Count, help_heading = "Logging")]
    pub verbose: u8,

    /// Log level
    #[clap(short, long, value_enum, default_value_t=StatusCode::Warn, help_heading="Logging")]
    pub loglevel: StatusCode,

    /// Be quiet, don’t report anything on the terminal.
    #[clap(short, long, help_heading = "Logging")]
    pub quiet: bool,

    /// This is a slightly more compact and succinct output layout
    #[clap(long, help_heading = "Logging")]
    pub succinct: bool,

    /// Timeout (in seconds) for network operations.
    #[clap(long, help_heading = "Network")]
    pub timeout: Option<u64>,

    /// Skip network checks
    #[clap(long, help_heading = "Network")]
    pub skip_network: bool,

    /// Write a JSON formatted report to the given filename
    #[clap(long, help_heading = "Reports")]
    pub json: Option<String>,

    /// Write a CSV formatted report to the given filename
    #[clap(long, help_heading = "Reports")]
    pub csv: Option<String>,

    /// Write run output to DuckDb database
    #[clap(long, help_heading = "Reports")]
    #[cfg(feature = "duckdb")]
    pub duckdb: Option<String>,

    /// Write a GitHub-Markdown formatted report to the given filename
    #[clap(long, help_heading = "Reports")]
    pub ghmarkdown: Option<String>,

    /// Write a HTML formatted report to the given filename
    #[clap(long, help_heading = "Reports")]
    pub html: Option<String>,

    /// Copy bundled templates to user template directory
    #[clap(long, help_heading = "Reports")]
    pub update_templates: bool,

    /// Write JSON badges to the given directory
    #[clap(long, help_heading = "Reports")]
    pub badges: Option<String>,

    /// Fall back to Python implementations of unported checks
    #[clap(long)]
    #[cfg(feature = "python")]
    pub use_python: bool,

    /// Hotfix found problems in the binaries
    #[clap(long, help_heading = "Fix problems")]
    pub hotfix: bool,

    /// Fix sources
    #[clap(long, help_heading = "Fix problems")]
    pub fix_sources: bool,

    #[arg(long, help_heading = "Fix problems", value_parser = parse_source_map)]
    pub source_map: HashMap<String, String>,

    /// Input files
    pub inputs: Vec<String>,
}

fn parse_source_map(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid source map entry (should be binary_file.ttf:source.glyphs): {s}"
        ));
    }
    #[allow(clippy::indexing_slicing)] // We know there are exactly two parts
    if parts[0].ends_with(".glyphs")
        || parts[0].ends_with(".glyphspackage")
        || parts[0].ends_with(".ufo")
        || parts[0].ends_with(".designspace")
        || parts[1].ends_with(".ttf")
        || parts[1].ends_with(".otf")
    {
        return Err(format!(
            "Invalid source map key (the binary font should go on the left): {s}",
        ));
    }
    #[allow(clippy::indexing_slicing)] // We know there are exactly two parts
    Ok((parts[0].to_string(), parts[1].to_string()))
}

//! Simple CLI tool for applying hotfixes to font binaries

#[cfg(feature = "cli")]
use clap::Parser;

#[cfg(feature = "cli")]
use fontspector_checkapi::Testable;

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(name = "fontspector-hotfix")]
#[command(about = "Apply font QA hotfixes to font binaries", long_about = None)]
struct Args {
    /// Path to the font file to fix
    #[arg(value_name = "FONT")]
    font: String,

    /// Check IDs to apply hotfixes for (can be specified multiple times)
    #[arg(short = 'c', long = "check", value_name = "CHECK_ID")]
    checks: Vec<String>,

    /// Profile to load checks from
    #[arg(short = 'p', long = "profile", value_name = "PROFILE")]
    profile: Option<String>,

    /// Output file (if not specified, overwrites input)
    #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
    output: Option<String>,

    /// Verbose output
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

#[cfg(feature = "cli")]
fn main() {
    let args = Args::parse();

    env_logger::init_from_env(env_logger::Env::default().filter_or(
        env_logger::DEFAULT_FILTER_ENV,
        if args.verbose { "info" } else { "warn" },
    ));

    let registry = fontspector_hotfix::get_registry();

    // Determine which check IDs to use
    let mut check_ids = args.checks.clone();

    // If a profile is specified, get all check IDs from it
    if let Some(profile_name) = &args.profile {
        let profile = registry.get_profile(profile_name).unwrap_or_else(|| {
            eprintln!("Error: Profile '{}' not found", profile_name);
            std::process::exit(1);
        });

        // Collect all check IDs from the profile
        for checks_in_section in profile.sections.values() {
            check_ids.extend(checks_in_section.iter().map(|s| s.to_string()));
        }

        log::info!(
            "Using {} checks from profile '{}'",
            check_ids.len(),
            profile_name
        );
    }

    if check_ids.is_empty() {
        eprintln!("Error: No check IDs specified. Use --check or --profile.");
        std::process::exit(1);
    }

    // Load the font
    let mut testable = Testable::new(&args.font).unwrap_or_else(|e| {
        eprintln!("Error: Could not load font '{}': {}", args.font, e);
        std::process::exit(1);
    });

    // Apply hotfixes
    match fontspector_hotfix::apply_hotfixes(&mut testable, &check_ids) {
        Ok(modified) => {
            if modified {
                // Save the file
                if let Some(output_path) = &args.output {
                    // Save to a different file
                    testable.filename = output_path.into();
                }

                testable.save().unwrap_or_else(|e| {
                    eprintln!("Error: Could not save font '{}': {}", args.font, e);
                    std::process::exit(1);
                });

                println!(
                    "Successfully applied hotfixes to '{}'",
                    args.output.as_ref().unwrap_or(&args.font)
                );
            } else {
                println!("No modifications were made to '{}'", args.font);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to apply hotfixes: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("This binary requires the 'cli' feature to be enabled.");
    std::process::exit(1);
}

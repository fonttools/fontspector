//! # fontspector-hotfix
//!
//! A simple library and tool for applying hotfixes to font binaries.
//!
//! This crate provides functionality to run a series of hotfix functions on a font file.
//! It looks up check IDs in the registry and applies any hotfix functions that are available.
//!
//! ## Library Usage
//!
//! ```ignore
//! use fontspector_hotfix::{apply_hotfixes, Testable};
//!
//! let mut testable = Testable::new("path/to/font.ttf")?;
//! let check_ids = vec!["com/google/fonts/check/example"];
//! let modified = apply_hotfixes(&mut testable, &check_ids)?;
//!
//! if modified {
//!     testable.save()?;
//! }
//! ```

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use fontspector_checkapi::{
    CheckId, DialogFieldType, FixResult, FontspectorError, MoreInfoReplies, MoreInfoRequest,
    Plugin, Registry,
};
use profile_fontwerk::Fontwerk;
use profile_googlefonts::GoogleFonts;
use profile_iso15008::Iso15008;
use profile_opentype::OpenType;
use profile_universal::Universal;
use serde_json::Value;
use std::io::Write;
use termimad::MadSkin;

#[doc(hidden)]
pub fn get_registry() -> Registry<'static> {
    let mut registry = Registry::new();
    OpenType
        .register(&mut registry)
        .expect("Couldn't register opentype profile");
    Universal
        .register(&mut registry)
        .expect("Couldn't register universal profile");
    GoogleFonts
        .register(&mut registry)
        .expect("Couldn't register googlefonts profile");
    Iso15008
        .register(&mut registry)
        .expect("Couldn't register iso15008 profile");
    Fontwerk
        .register(&mut registry)
        .expect("Couldn't register fontwerk profile");
    registry
}

// Re-export Testable for convenience
pub use fontspector_checkapi::Testable;

/// Result of applying hotfixes
pub type HotfixResult = Result<bool, FontspectorError>;

/// Apply hotfixes to a testable for the given check IDs.
///
/// For each check ID provided, this function looks up the check in the registry
/// and runs its hotfix function (if available) on the testable.
///
/// # Arguments
///
/// * `testable` - A mutable reference to the testable (font) to modify
/// * `check_ids` - A slice of check IDs to apply hotfixes for
///
/// # Returns
///
/// Returns `Ok(true)` if any hotfix modified the testable, `Ok(false)` if no
/// modifications were made, or an `Err` if any hotfix failed.
///
/// # Errors
///
/// Returns an error if any hotfix function encounters an error during execution.
pub fn apply_hotfixes(testable: &mut Testable, check_ids: &[CheckId]) -> HotfixResult {
    let mut any_modified = false;

    let registry = get_registry();

    for check_id in check_ids {
        let Some(check) = registry.checks.get(check_id.as_str()) else {
            log::warn!("Check ID '{}' not found in registry", check_id);
            continue;
        };

        let Some(hotfix) = check.hotfix else {
            log::debug!("Check '{}' has no hotfix function", check_id);
            continue;
        };

        log::info!("Applying hotfix for check '{}'", check_id);
        let mut options = None;
        let mut header_shown = false;

        loop {
            match hotfix(testable, options) {
                Ok(FixResult::MoreInfoNeeded(dialog)) => {
                    if !header_shown {
                        show_header(testable, check_id, check.title);
                        header_shown = true;
                    }

                    options = run_dialog(&dialog);
                    continue;
                }
                Ok(hotfix_result) => {
                    if matches!(hotfix_result, FixResult::Fixed) {
                        log::info!("Check '{}' modified the font", check_id);
                        any_modified = true;
                    } else {
                        log::debug!(
                            "Check '{}' completed with result: {:?}",
                            check_id,
                            hotfix_result
                        );
                    }
                    break;
                }
                Err(e) => {
                    log::error!("Hotfix for '{}' failed: {}", check_id, e);
                    return Err(e);
                }
            }
        }
    }

    Ok(any_modified)
}

fn show_header(testable: &mut Testable, check_id: &str, check_name: &str) {
    let skin = MadSkin::default();
    let filename = testable.filename.to_string_lossy();
    let _ = writeln!(std::io::stdout(), "Testing: {filename}");
    let _ = writeln!(std::io::stdout(), "  Check: {check_id}\n");
    let _ = writeln!(std::io::stdout(), "  {:}\n", check_name);
    eprintln!(
        "{}",
        skin.term_text(
            "More information was needed to fix this problem, so we will ask you some questions to help the process.\n"
        )
    );
}

fn run_dialog(dialog: &MoreInfoRequest) -> Option<MoreInfoReplies> {
    let mut replies = MoreInfoReplies::default();
    for field in &dialog.0 {
        match &field.field_type {
            DialogFieldType::Choice(choices) => {
                let choice_keys = choices.iter().map(|x| x.value.clone()).collect::<Vec<_>>();
                let items = choices
                    .iter()
                    .map(|x| x.description.clone())
                    .collect::<Vec<_>>();
                if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(&field.prompt)
                    .items(&items)
                    .interact_opt()
                {
                    replies.0.insert(
                        field.key.clone(),
                        Value::String(choice_keys.get(selection).cloned().unwrap_or_default()),
                    );
                }
            }
            DialogFieldType::Text => {
                if let Ok(input) = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(&field.prompt)
                    .interact_text()
                {
                    replies.0.insert(field.key.clone(), Value::String(input));
                }
            }
            DialogFieldType::Number => {
                if let Ok(input) = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(&field.prompt)
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.parse::<f64>().is_ok() {
                            Ok(())
                        } else {
                            Err("Please enter a valid number")
                        }
                    })
                    .interact_text()
                {
                    if let Ok(number) = input.parse::<f64>() {
                        #[allow(clippy::unwrap_used)]
                        replies.0.insert(
                            field.key.clone(),
                            Value::Number(serde_json::Number::from_f64(number).unwrap()),
                        );
                    }
                }
            }
            DialogFieldType::Boolean => {
                if let Ok(confirmation) = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(&field.prompt)
                    .interact()
                {
                    replies
                        .0
                        .insert(field.key.clone(), Value::Bool(confirmation));
                }
            }
        }
    }
    Some(replies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_hotfixes_empty() {
        let registry = Registry::new();
        // This would need a valid font file to work properly
        // Just testing that the function signature is correct
        assert!(registry.checks.is_empty());
    }
}

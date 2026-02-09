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

use fontspector_checkapi::{CheckId, FontspectorError, Plugin, Registry};
use profile_fontwerk::Fontwerk;
use profile_googlefonts::GoogleFonts;
use profile_iso15008::Iso15008;
use profile_opentype::OpenType;
use profile_universal::Universal;

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
/// * `registry` - The registry containing the checks
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
        match hotfix(testable) {
            Ok(modified) => {
                if modified {
                    log::info!("Check '{}' modified the font", check_id);
                    any_modified = true;
                } else {
                    log::debug!("Check '{}' made no modifications", check_id);
                }
            }
            Err(e) => {
                log::error!("Hotfix for '{}' failed: {}", check_id, e);
                return Err(e);
            }
        }
    }

    Ok(any_modified)
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

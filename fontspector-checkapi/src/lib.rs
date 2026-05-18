#![deny(missing_docs, clippy::missing_docs_in_private_items)]

//! # fontspector-checkapi
//!
//! This crate provides an API for writing checks for fontspector. It is used by
//! check implementations in the various `profile-` crates. As well as the interfae
//! for "talking" to fontspector, it provides some useful functionality for
//! writing checks on TrueType font files.
//!
//! This crate also exports a `prelude` module containing the most common items you will
//! need when writing checks.
//!
//! Check authors should see also [fontspector-checkhelper](../fontspector-checkhelper)

/// Routines and data structures for defining a check
mod check;
/// Data structures representing the result of a check
mod checkresult;
/// Routines for testing checks
pub mod codetesting;
/// Font-related constants which may be useful to check implementors
pub mod constants;
/// Data structures for managing the context in which a check is run
mod context;
/// Error types
mod error;

/// Managing a registry of file types
mod filetype;
/// Data types for applying fixes to fonts
mod fix;
/// Represents a TrueType font, together with useful routines for dealing with them
mod font;
/// Routines to make dealing with GSUB tables more tractable
mod gsub;
/// [OutlinePen](https://docs.rs/skrifa/latest/skrifa/outline/trait.OutlinePen.html) implementations useful for check implementors
pub mod pens;
/// Utilities for building fontspector plugins
pub mod plugin;
/// Sets of checks that declare a particular "standard" of QA testing
mod profile;
/// The registry of checks and profiles
mod registry;
/// Data structures representing the most basic elements of a check's result
mod status;
/// Wraps a file or "thing" to be tested
mod testable;
/// Common utility functions for check implementors
mod utils;
pub use check::{return_result, Check, CheckFlags, CheckId, CheckImplementation};
pub use checkresult::CheckResult;
pub use context::Context;
pub use error::FontspectorError;
pub use filetype::{FileType, FileTypeConvert};
pub use fix::{
    Choice, DialogField, DialogFieldType, FixResult, HotfixFunction, MoreInfoReplies,
    MoreInfoRequest,
};
pub use font::{
    get_name_entry_string, get_name_platform_tuples, PlatformSelector, TestFont, DEFAULT_LOCATION,
    TTF,
};
pub use gsub::{GetSubstitutionMap, SubstitutionMap};
pub use profile::{Override, Profile, ProfileBuilder};
pub use registry::Registry;
pub use status::{CheckFnResult, Metadata, Status, StatusCode, StatusList};
pub use testable::{Testable, TestableCollection, TestableType};

/// The prelude module contains the most common items you will need when writing checks
pub mod prelude {
    pub use fontspector_checkhelper::check;

    #[macro_export]
    /// Extract a TTF file from a [Testable] or return an error if you can't
    macro_rules! testfont {
        ($f: ident) => {
            TTF.from_testable($f)
                .ok_or(FontspectorError::InappropriateFile {
                    expected: "TTF",
                    filename: $f.filename.to_string_lossy().to_string(),
                    more_details: "Not a TTF file".to_string(),
                })?
        };
    }
    /// Return a skip status with a code and message
    ///
    /// This macro has two forms:
    /// `skip!(code, message)` which will always return a skip status, and
    /// `skip!(condition, code, message)` which will return a skip status if the condition is true
    #[macro_export]
    macro_rules! skip {
        ($code: expr, $message: expr) => {
            return Ok(Status::just_one_skip($code, $message));
        };
        ($condition: expr, $code: expr, $message: expr) => {
            if $condition {
                return Ok(Status::just_one_skip($code, $message));
            }
        };
    }
    /// The expected return type of a hotfix function
    pub use crate::{
        return_result, utils::*, Check, CheckFlags, CheckFnResult, CheckImplementation, Context,
        FileType, FixResult, FontspectorError, MoreInfoReplies, MoreInfoRequest, Profile,
        ProfileBuilder, Registry, Status, StatusList, Testable, TestableCollection, TestableType,
        TTF,
    };
}

/// Something - a plugin or a crate - which provides profiles and checks to be registered with the fontspector registry.
///
/// Plugins contain checks and profiles that can be registered with the fontspector
/// registry. The plugin must implement this trait and provide a function that
/// returns an instance of the plugin.
pub trait ProfileProvider {
    /// Register the checks and profiles in the plugin with the registry
    fn register(&self, cr: &mut Registry) -> Result<(), FontspectorError>;
}

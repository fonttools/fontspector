#![deny(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    clippy::missing_docs_in_private_items
)]
//! Checks in fontspector are made up of two parts: the implementation,
//! which is a Rust function, and metadata: the check's ID, rationale,
//! proposal URL(s) and so on. This crate provides a procedural macro to
//! help associate the metadata with the implementation.
//!
//! To write a fontspector check, you need to implement a function that
//! takes a `Testable` and a `Context` as arguments (for checks which operate
//! on a single font) or a `TestableCollection` and a `Context`. You
//! then decorate the function with the `#[check(...)]` macro, passing
//! the metadata as arguments. The macro will generate the necessary
//! code to associate the metadata with the implementation.
//!
//! A function which takes a `TestableCollection` should have the argument
//! `implementation = "all"` in the `#[check(...)]` attribute. This will

/// Check parser and macro implementation
mod check;
use proc_macro::TokenStream;

use check::check_impl;

/// A procedural macro to associate metadata with a check implementation.
///
/// Example:
///
/// ```ignore
/// use fontspector_api::prelude::*; // Includes this macro
///
/// #[check(
///    id = "example_check",
///   title = "Example Check",
///   rationale = "This is an example check.",
/// ///   proposal = "https://example.com/proposal",
/// )]
/// fn example_check(f: &Testable, _context: &Context) -> CheckFnResult {
///    // Check implementation goes here
/// }
/// ```
#[proc_macro_attribute]
pub fn check(args: TokenStream, item: TokenStream) -> TokenStream {
    check_impl(args, item)
}

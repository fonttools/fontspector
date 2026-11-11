mod known_names;
mod merge;
mod names;
mod resolve;
mod token;

pub use merge::{family_naming_info, StyleParts};
pub use names::{extract, name_string_with_fallback, FontNaming, NameStrings};
pub use resolve::{resolve_names, ResolvedStyle};
pub use token::get_name_tuples;

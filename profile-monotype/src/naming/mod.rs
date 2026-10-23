mod known_names;
mod merge;
mod resolve;
mod token;

pub use merge::{family_naming_info, StyleParts};
pub use resolve::{resolve_names, ResolvedStyle};
pub use token::get_name_tuples;

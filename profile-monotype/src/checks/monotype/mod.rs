#[allow(non_snake_case)]
// TODO: add Monotype-specific checks
mod fstype;
pub use fstype::fstype;
mod names;
pub use names::style_in_family_name;
mod widthclass;
pub use widthclass::widthclass;
mod unnecessary_tables;
pub use unnecessary_tables::unnecessary_tables;

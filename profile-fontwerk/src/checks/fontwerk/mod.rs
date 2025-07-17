#[allow(non_snake_case)]
mod names;
pub use names::name_consistency;
pub use names::name_entries;
pub use names::required_name_ids;
mod soft_hyphen;
pub use soft_hyphen::soft_hyphen;

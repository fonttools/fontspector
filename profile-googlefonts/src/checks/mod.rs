mod dotted_circle;
pub mod googlefonts;
#[cfg(feature = "check")]
pub mod outline;
pub use dotted_circle::dotted_circle;
#[cfg(all(feature = "check", not(target_family = "wasm")))]
// Because it needs to find *.json. XXX Rewrite to use collections
pub mod shaping;
#[cfg(feature = "check")]
mod soft_dotted;
#[cfg(feature = "check")]
pub use soft_dotted::soft_dotted;

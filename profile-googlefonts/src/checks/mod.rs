mod dotted_circle;
pub mod googlefonts;
pub mod outline;
pub use dotted_circle::dotted_circle;
#[cfg(not(target_family = "wasm"))]
// Because it needs to find *.json. XXX Rewrite to use collections
pub mod shaping;
mod soft_dotted;
pub use soft_dotted::soft_dotted;

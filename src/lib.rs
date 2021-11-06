mod params;
mod service;
mod async_utils;
mod output;

pub use params::Params;
pub use async_utils::*;
pub use output::*;
pub use service::Core;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


pub const LOGO: &str = r"
RustScanner v1.0.1
A Simple Port Scanner Written In Rust
https://github.com/krishpranav/rustscanner
";
//! OpenQASM 3.0 formatter

pub mod comment;
pub mod config;
pub mod error;
pub mod format;
pub mod ir;
pub mod printer;

pub use config::FormatConfig;
pub use error::FormatError;

pub fn format(source: &str) -> Result<String, FormatError> {
    format_with_config(source, FormatConfig::default())
}

pub fn format_with_config(source: &str, config: FormatConfig) -> Result<String, FormatError> {
    format::format_source(source, &config)
}

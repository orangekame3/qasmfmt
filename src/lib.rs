pub mod config;
pub mod error;

pub fn format(source: &str) -> Result<String, error::FormatError> {
    Ok(source.to_string())
}

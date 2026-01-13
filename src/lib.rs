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

#[cfg(feature = "python")]
mod python {
    use pyo3::prelude::*;

    use crate::config::FormatConfig;

    #[pyfunction]
    #[pyo3(signature = (source, indent_size=4, max_width=100))]
    fn format_str(source: &str, indent_size: usize, max_width: usize) -> PyResult<String> {
        let config = FormatConfig {
            indent_size,
            max_width,
            ..Default::default()
        };
        crate::format_with_config(source, config)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    #[pymodule]
    #[pyo3(name = "qasmfmt")]
    fn qasmfmt_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(format_str, m)?)?;
        m.add("__version__", env!("CARGO_PKG_VERSION"))?;
        Ok(())
    }
}

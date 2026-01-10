use thiserror::Error;

#[derive(Debug, Error)]
pub enum FormatError {
    #[error("parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
    #[error("empty input")]
    EmptyInput,

    #[error("syntax error: {0}")]
    Syntax(String),
}

use std::io;

#[derive(thiserror::Error, Debug)]
pub enum RoxError {
    #[error("[line {line}] SyntaxError: {message}")]
    SyntaxError { line: usize, message: String },
    #[error(transparent)]
    IOError(#[from] io::Error),
}

pub type RoxResult<T> = Result<T, RoxError>;

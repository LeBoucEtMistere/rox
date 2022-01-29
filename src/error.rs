use std::io;

#[derive(thiserror::Error, Debug)]
pub enum InternalRoxError {
    #[error("[line {line}] SyntaxError: {message}")]
    SyntaxError { line: usize, message: String },
}

#[derive(thiserror::Error, Debug)]
pub enum FacingRoxError {
    #[error("Syntax Error")]
    SyntaxError,
    #[error(transparent)]
    IOError(#[from] io::Error),
}

pub type InternalRoxResult<T> = Result<T, InternalRoxError>;
pub type FacingRoxResult<T> = Result<T, FacingRoxError>;

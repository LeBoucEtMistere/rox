use std::io;

use crate::parser::error::ParseError;

#[derive(thiserror::Error, Debug)]
pub enum InternalRoxError {
    #[error("[line {line}] SyntaxError: {message}")]
    SyntaxError { line: usize, message: String },
}
#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum FacingRoxError {
    #[error("Syntax Error")]
    SyntaxError,
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    ParseError(#[from] ParseError),
}

pub type InternalRoxResult<T> = Result<T, InternalRoxError>;
pub type FacingRoxResult<T> = Result<T, FacingRoxError>;

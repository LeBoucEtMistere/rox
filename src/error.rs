use std::io;

use crate::{
    interpreter::error::InterpreterError,
    parser::error::ParserError,
    scanner::error::ScannerError,
};

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum FacingRoxError {
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    ParserError(#[from] ParserError),
    #[error(transparent)]
    ScannerError(#[from] ScannerError),
    #[error(transparent)]
    InterpreterError(#[from] InterpreterError),
}

pub type FacingRoxResult<T> = Result<T, FacingRoxError>;
pub type FacingRoxResults<T> = Result<T, Vec<FacingRoxError>>;

use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub struct ScannerError {
    line_index: usize,
    msg: String,
}

impl ScannerError {
    pub fn new(line_index: usize, msg: String) -> Self {
        Self { line_index, msg }
    }
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scanning Error - line {}: {}", self.line_index, self.msg)?;

        Ok(())
    }
}

pub type ScannerResult<T> = Result<T, ScannerError>;
pub type ScannerResults<T> = Result<T, Vec<ScannerError>>;

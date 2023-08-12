use std::fmt::Display;

use thiserror::Error;

use crate::token::{Token, TokenType};
#[derive(Error, Debug, PartialEq)]
pub struct ParserError {
    token: Token,
    msg: String,
}

impl ParserError {
    pub fn new(token: Token, msg: String) -> Self {
        Self { token, msg }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.token.token_type == TokenType::Eof {
            write!(
                f,
                "Parsing Error - line {} at end: {}",
                self.token.line, self.msg
            )?;
        } else {
            write!(
                f,
                "Parsing Error - line {} at {}: {}",
                self.token.line, self.token.lexeme, self.msg
            )?;
        }
        Ok(())
    }
}

pub type ParserResults<T> = Result<T, Vec<ParserError>>;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("TypeError: {0}")]
    TypeError(String),
}

pub type InterpreterResult<T> = Result<T, InterpreterError>;

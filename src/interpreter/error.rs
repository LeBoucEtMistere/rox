use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("TypeError: {0}")]
    TypeError(String),
    #[error("RuntimeError: {0}")]
    RuntimeError(String),
}

pub type InterpreterResult<T> = Result<T, InterpreterError>;

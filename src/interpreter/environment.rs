use std::collections::HashMap;

use super::{
    error::{InterpreterError, InterpreterResult},
    EvaluatedExpr,
};
use crate::token::Token;

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, EvaluatedExpr>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: EvaluatedExpr) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> InterpreterResult<EvaluatedExpr> {
        self.values
            .get(&name.lexeme)
            .cloned()
            .ok_or(InterpreterError::RuntimeError(format!(
                "Undefined variable {}",
                name.lexeme
            )))
    }
}

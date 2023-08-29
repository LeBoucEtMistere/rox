use std::collections::HashMap;

use super::{
    error::{InterpreterError, InterpreterResult},
    EvaluatedExpr,
};
use crate::token::Token;

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, EvaluatedExpr>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn set_enclosing(&mut self, enclosing: Environment) {
        if self.enclosing.is_some() {
            panic!("Cannot set enclosing envirnoment out as it's already set")
        }
        self.enclosing = Some(Box::new(enclosing))
    }

    pub fn take_enclosing(&mut self) -> Environment {
        let ret = self.enclosing.take();
        if let Some(b) = ret {
            *b
        } else {
            panic!("Cannot take enclosing envirnoment out as it's not set")
        }
    }

    pub fn define(&mut self, name: String, value: EvaluatedExpr) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> InterpreterResult<EvaluatedExpr> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).cloned().unwrap())
        } else {
            if let Some(enclosing) = self.enclosing.as_ref() {
                return enclosing.get(name);
            }
            Err(InterpreterError::RuntimeError(format!(
                "Undefined variable {}",
                name.lexeme
            )))
        }
    }

    pub fn assign(&mut self, name: &Token, value: EvaluatedExpr) -> InterpreterResult<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            Err(InterpreterError::RuntimeError(format!(
                "Undefined variable '{}'",
                name.lexeme
            )))
        }
    }
}

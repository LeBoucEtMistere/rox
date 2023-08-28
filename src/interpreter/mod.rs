pub mod environment;
pub mod error;

use self::{
    environment::Environment,
    error::{InterpreterError, InterpreterResult},
};
use crate::{
    ast::{
        expression::{Binary, Grouping, Literal, Unary, Variable},
        statement::{ExpressionStatement, PrintStatement, VariableStatement},
        visitor::{ExprVisitor, StatementVisitor},
        Expr,
        Statement,
    },
    token::TokenType,
};

#[derive(Debug, PartialEq, Clone)]
pub enum EvaluatedExpr {
    Nil,
    String(String),
    Number(f64),
    Boolean(bool),
}

impl ToString for EvaluatedExpr {
    fn to_string(&self) -> String {
        match self {
            EvaluatedExpr::Nil => "nil".to_string(),
            EvaluatedExpr::String(v) => v.to_string(),
            EvaluatedExpr::Number(v) => v.to_string(),
            EvaluatedExpr::Boolean(v) => v.to_string(),
        }
    }
}

#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn interpret(&mut self, statements: &[Statement]) -> InterpreterResult<()> {
        for s in statements.iter() {
            self.execute(s)?
        }
        Ok(())
    }
    fn evaluate(&mut self, expr: &Expr) -> InterpreterResult<EvaluatedExpr> {
        expr.accept(self)
    }
    fn execute(&mut self, statement: &Statement) -> InterpreterResult<()> {
        statement.accept(self)
    }
}

fn is_truthy(value: &EvaluatedExpr) -> bool {
    match value {
        EvaluatedExpr::Nil => false,
        EvaluatedExpr::String(_) => true,
        EvaluatedExpr::Number(_) => true,
        EvaluatedExpr::Boolean(b) => *b,
    }
}

impl ExprVisitor for Interpreter {
    type Return = InterpreterResult<EvaluatedExpr>;

    fn visit_unary(&mut self, unary: &Unary) -> Self::Return {
        let evaluated_right = self.evaluate(&unary.expr)?;

        match unary.op.token_type {
            TokenType::Minus => {
                if let EvaluatedExpr::Number(v) = evaluated_right {
                    Ok(EvaluatedExpr::Number(-v))
                } else {
                    Err(InterpreterError::TypeError(
                        "Expected f64 after unary operator -".into(),
                    ))
                }
            }
            TokenType::Bang => Ok(EvaluatedExpr::Boolean(!is_truthy(&evaluated_right))),
            t => Err(InterpreterError::TypeError(format!(
                "Operand {t:?} not supported in unary expression"
            ))),
        }
    }

    fn visit_binary(&mut self, binary: &Binary) -> Self::Return {
        let evaluated_left = self.evaluate(&binary.left)?;
        let evaluated_right = self.evaluate(&binary.right)?;

        match binary.op.token_type {
            TokenType::Minus => {
                if let EvaluatedExpr::Number(l) = evaluated_left {
                    if let EvaluatedExpr::Number(r) = evaluated_right {
                        Ok(EvaluatedExpr::Number(l - r))
                    } else {
                        Err(InterpreterError::TypeError(
                            "Right of - binary should be a valid number".into(),
                        ))
                    }
                } else {
                    Err(InterpreterError::TypeError(
                        "Left of - binary should be a valid number".into(),
                    ))
                }
            }
            TokenType::Slash => {
                if let EvaluatedExpr::Number(l) = evaluated_left {
                    if let EvaluatedExpr::Number(r) = evaluated_right {
                        Ok(EvaluatedExpr::Number(l / r))
                    } else {
                        Err(InterpreterError::TypeError(
                            "Right of / binary should be a valid number".into(),
                        ))
                    }
                } else {
                    Err(InterpreterError::TypeError(
                        "Left of / binary should be a valid number".into(),
                    ))
                }
            }
            TokenType::Star => {
                if let EvaluatedExpr::Number(l) = evaluated_left {
                    if let EvaluatedExpr::Number(r) = evaluated_right {
                        Ok(EvaluatedExpr::Number(l * r))
                    } else {
                        Err(InterpreterError::TypeError(
                            "Right of * binary should be a valid number".into(),
                        ))
                    }
                } else {
                    Err(InterpreterError::TypeError(
                        "Left of * binary should be a valid number".into(),
                    ))
                }
            }
            TokenType::Plus => match evaluated_left {
                EvaluatedExpr::Number(l) => {
                    if let EvaluatedExpr::Number(r) = evaluated_right {
                        Ok(EvaluatedExpr::Number(l + r))
                    } else {
                        Err(InterpreterError::TypeError(
                            "Right of + binary should be a valid number when left is a number"
                                .into(),
                        ))
                    }
                }
                EvaluatedExpr::String(l) => {
                    if let EvaluatedExpr::String(r) = evaluated_right {
                        Ok(EvaluatedExpr::String(format!("{l}{r}")))
                    } else {
                        Err(InterpreterError::TypeError(
                            "Right of + binary should be a valid string when left is a string"
                                .into(),
                        ))
                    }
                }
                _ => Err(InterpreterError::TypeError(
                    "Cannot evaluate + operand, left expression should be a string or number"
                        .into(),
                )),
            },
            TokenType::Greater => {
                if let EvaluatedExpr::Number(l) = evaluated_left {
                    if let EvaluatedExpr::Number(r) = evaluated_right {
                        Ok(EvaluatedExpr::Boolean(l > r))
                    } else {
                        Err(InterpreterError::TypeError(
                            "Right of > binary should be a valid number".into(),
                        ))
                    }
                } else {
                    Err(InterpreterError::TypeError(
                        "Left of > binary should be a valid number".into(),
                    ))
                }
            }
            TokenType::GreaterEqual => {
                if let EvaluatedExpr::Number(l) = evaluated_left {
                    if let EvaluatedExpr::Number(r) = evaluated_right {
                        Ok(EvaluatedExpr::Boolean(l >= r))
                    } else {
                        Err(InterpreterError::TypeError(
                            "Right of >= binary should be a valid number".into(),
                        ))
                    }
                } else {
                    Err(InterpreterError::TypeError(
                        "Left of >= binary should be a valid number".into(),
                    ))
                }
            }
            TokenType::Less => {
                if let EvaluatedExpr::Number(l) = evaluated_left {
                    if let EvaluatedExpr::Number(r) = evaluated_right {
                        Ok(EvaluatedExpr::Boolean(l < r))
                    } else {
                        Err(InterpreterError::TypeError(
                            "Right of < binary should be a valid number".into(),
                        ))
                    }
                } else {
                    Err(InterpreterError::TypeError(
                        "Left of < binary should be a valid number".into(),
                    ))
                }
            }
            TokenType::LessEqual => {
                if let EvaluatedExpr::Number(l) = evaluated_left {
                    if let EvaluatedExpr::Number(r) = evaluated_right {
                        Ok(EvaluatedExpr::Boolean(l <= r))
                    } else {
                        Err(InterpreterError::TypeError(
                            "Right of <= binary should be a valid number".into(),
                        ))
                    }
                } else {
                    Err(InterpreterError::TypeError(
                        "Left of <= binary should be a valid number".into(),
                    ))
                }
            }
            TokenType::EqualEqual => Ok(EvaluatedExpr::Boolean(evaluated_left == evaluated_right)),
            TokenType::BangEqual => Ok(EvaluatedExpr::Boolean(evaluated_left != evaluated_right)),
            t => Err(InterpreterError::TypeError(format!(
                "Operand {t:?} not supported in binary expression"
            ))),
        }
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Return {
        self.evaluate(&grouping.expr)
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Return {
        Ok(match literal {
            Literal::Boolean(v) => EvaluatedExpr::Boolean(*v),
            Literal::String(v) => EvaluatedExpr::String(v.clone()),
            Literal::Nil => EvaluatedExpr::Nil,
            Literal::Number(v) => EvaluatedExpr::Number(*v),
        })
    }

    fn visit_variable(&mut self, variable: &Variable) -> Self::Return {
        self.environment.get(&variable.name)
    }
}

impl StatementVisitor for Interpreter {
    type Return = InterpreterResult<()>;

    fn visit_print(&mut self, statement: &PrintStatement) -> Self::Return {
        let value = self.evaluate(&statement.expr)?;
        println!("{}", value.to_string());
        Ok(())
    }

    fn visit_expression(&mut self, statement: &ExpressionStatement) -> Self::Return {
        self.evaluate(&statement.expr)?;
        Ok(())
    }

    fn visit_variable(&mut self, variable: &VariableStatement) -> Self::Return {
        let mut value = EvaluatedExpr::Nil;
        if let Some(init) = variable.initializer.as_ref() {
            value = self.evaluate(init)?;
        }
        self.environment.define(variable.name.lexeme.clone(), value);
        Ok(())
    }
}

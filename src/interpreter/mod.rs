pub mod error;

use std::any::Any;

use self::error::{InterpreterError, InterpreterResult, InterpreterResults};
use crate::{
    ast::{
        expression::{Binary, Grouping, Literal, Unary},
        visitor::ExprVisitor,
        Expr,
    },
    token::TokenType,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&mut self, expr: &Expr) -> InterpreterResults<String> {
        let boxed_result = self.evaluate(expr).map_err(|err| vec![err])?;

        Ok(if let Some(v) = boxed_result.downcast_ref::<bool>() {
            format!("{v}")
        } else if boxed_result.is::<()>() {
            "nil".to_owned()
        } else if let Some(v) = boxed_result.downcast_ref::<f64>() {
            format!("{v}")
        } else if let Ok(v) = boxed_result.downcast::<String>() {
            *v
        } else {
            panic!()
        })
    }
    fn evaluate(&mut self, expr: &Expr) -> InterpreterResult<Box<dyn Any>> {
        expr.accept(self)
    }

    fn is_truthy(value: &dyn Any) -> bool {
        if let Some(&v) = value.downcast_ref::<bool>() {
            v
        } else {
            // return false if it's a nil (Box<()>), else true
            !matches!(value.downcast_ref::<()>(), Some(_))
        }
    }
}

fn is_equal(left: &dyn Any, right: &dyn Any) -> InterpreterResult<bool> {
    if (*left).type_id() != (*right).type_id() {
        // different types cannot be equal
        Ok(false)
    } else if left.is::<()>() {
        // nil is always equal to itself
        Ok(true)
    } else if left.is::<String>() {
        Ok(left.downcast_ref::<String>().unwrap() == right.downcast_ref::<String>().unwrap())
    } else if left.is::<f64>() {
        Ok(left.downcast_ref::<f64>().unwrap() == right.downcast_ref::<f64>().unwrap())
    } else if left.is::<bool>() {
        Ok(left.downcast_ref::<bool>().unwrap() == right.downcast_ref::<bool>().unwrap())
    } else {
        Err(InterpreterError::TypeError(
            "Trying to compute equality of an unknown type".into(),
        ))
    }
}

impl<'a> ExprVisitor<'a> for Interpreter {
    type Return = InterpreterResult<Box<dyn Any>>;

    fn visit_unary(&mut self, unary: &'a Unary) -> Self::Return {
        let mut boxed_right = self.evaluate(&unary.expr)?;

        match unary.op.token_type {
            TokenType::Minus => Ok(Box::new(
                boxed_right.downcast_mut::<f64>().map(|&mut v| -v).ok_or(
                    InterpreterError::TypeError("Expected f64 after unary operator -".into()),
                )?,
            )),
            TokenType::Bang => Ok(Box::new(!Interpreter::is_truthy(&boxed_right))),
            t => Err(InterpreterError::TypeError(format!(
                "Operand {t:?} not supported in unary expression"
            ))),
        }
    }

    fn visit_binary(&mut self, binary: &'a Binary) -> Self::Return {
        let boxed_left = self.evaluate(&binary.left)?;
        let boxed_right = self.evaluate(&binary.right)?;

        match binary.op.token_type {
            TokenType::Minus => Ok(Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .ok_or(InterpreterError::TypeError(
                        "Left of - binary should be a valid number".into(),
                    ))?
                    - boxed_right
                        .downcast_ref::<f64>()
                        .ok_or(InterpreterError::TypeError(
                            "Right of - binary should be a valid number".into(),
                        ))?,
            )),
            TokenType::Slash => Ok(Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .ok_or(InterpreterError::TypeError(
                        "Left of / binary should be a valid number".into(),
                    ))?
                    / boxed_right
                        .downcast_ref::<f64>()
                        .ok_or(InterpreterError::TypeError(
                            "Right of / binary should be a valid number".into(),
                        ))?,
            )),
            TokenType::Star => Ok(Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .ok_or(InterpreterError::TypeError(
                        "Left of * binary should be a valid number".into(),
                    ))?
                    * boxed_right
                        .downcast_ref::<f64>()
                        .ok_or(InterpreterError::TypeError(
                            "Right of * binary should be a valid number".into(),
                        ))?,
            )),
            TokenType::Plus => {
                // + can add together two numbers or two strings
                if boxed_left.is::<f64>() && boxed_right.is::<f64>() {
                    Ok(Box::new(
                        boxed_left.downcast_ref::<f64>().unwrap()
                            + boxed_right.downcast_ref::<f64>().unwrap(),
                    ))
                } else if boxed_left.is::<String>() && boxed_right.is::<String>() {
                    Ok(Box::new(format!(
                        "{}{}",
                        boxed_left.downcast_ref::<String>().unwrap(),
                        boxed_right.downcast_ref::<f64>().unwrap()
                    )))
                } else {
                    Err(InterpreterError::TypeError(
                        "Cannot evaluate + operand, both expressions should be strings or numbers"
                            .into(),
                    ))
                }
            }
            TokenType::Greater => Ok(Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .ok_or(InterpreterError::TypeError(
                        "Left of > binary should be a valid number".into(),
                    ))?
                    > boxed_right
                        .downcast_ref::<f64>()
                        .ok_or(InterpreterError::TypeError(
                            "Right of > binary should be a valid number".into(),
                        ))?,
            )),
            TokenType::GreaterEqual => Ok(Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .ok_or(InterpreterError::TypeError(
                        "Left of >= binary should be a valid number".into(),
                    ))?
                    >= boxed_right
                        .downcast_ref::<f64>()
                        .ok_or(InterpreterError::TypeError(
                            "Right of >= binary should be a valid number".into(),
                        ))?,
            )),
            TokenType::Less => Ok(Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .ok_or(InterpreterError::TypeError(
                        "Left of < binary should be a valid number".into(),
                    ))?
                    < boxed_right
                        .downcast_ref::<f64>()
                        .ok_or(InterpreterError::TypeError(
                            "Right of < binary should be a valid number".into(),
                        ))?,
            )),
            TokenType::LessEqual => Ok(Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .ok_or(InterpreterError::TypeError(
                        "Left of <= binary should be a valid number".into(),
                    ))?
                    <= boxed_right
                        .downcast_ref::<f64>()
                        .ok_or(InterpreterError::TypeError(
                            "Right of <= binary should be a valid number".into(),
                        ))?,
            )),
            TokenType::EqualEqual => Ok(Box::new(is_equal(&*boxed_left, &*boxed_right)?)),
            TokenType::BangEqual => Ok(Box::new(!is_equal(&*boxed_left, &*boxed_right)?)),
            t => Err(InterpreterError::TypeError(format!(
                "Operand {t:?} not supported in binary expression"
            ))),
        }
    }

    fn visit_grouping(&mut self, grouping: &'a Grouping) -> Self::Return {
        self.evaluate(&grouping.expr)
    }

    fn visit_literal(&mut self, literal: &'a Literal) -> Self::Return {
        match literal {
            Literal::Boolean(v) => Ok(Box::new(*v)),
            Literal::String(v) => Ok(Box::new(v.clone())),
            Literal::Nil => Ok(Box::new(())),
            Literal::Number(v) => Ok(Box::new(*v)),
        }
    }
}

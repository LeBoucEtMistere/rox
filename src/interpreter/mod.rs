use std::any::Any;

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
    pub fn interpret(&mut self, expr: &Expr) -> String {
        let boxed_result = self.evaluate(expr);

        if let Some(v) = boxed_result.downcast_ref::<bool>() {
            format!("{v}")
        } else if boxed_result.is::<()>() {
            "nil".to_owned()
        } else if let Some(v) = boxed_result.downcast_ref::<f64>() {
            format!("{v}")
        } else if let Ok(v) = boxed_result.downcast::<String>() {
            *v
        } else {
            panic!()
        }
    }
    fn evaluate(&mut self, expr: &Expr) -> Box<dyn Any> {
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

fn is_equal(left: &dyn Any, right: &dyn Any) -> bool {
    if (*left).type_id() != (*right).type_id() {
        // different types cannot be equal
        false
    } else if left.is::<()>() {
        // nil is always equal to itself
        true
    } else if left.is::<String>() {
        left.downcast_ref::<String>().unwrap() == right.downcast_ref::<String>().unwrap()
    } else if left.is::<f64>() {
        left.downcast_ref::<f64>().unwrap() == right.downcast_ref::<f64>().unwrap()
    } else if left.is::<bool>() {
        left.downcast_ref::<bool>().unwrap() == right.downcast_ref::<bool>().unwrap()
    } else {
        panic!("Don't know how to compare two types for equality")
    }
}

impl<'a> ExprVisitor<'a> for Interpreter {
    type Return = Box<dyn Any>;

    fn visit_unary(&mut self, unary: &'a Unary) -> Self::Return {
        let mut boxed_right = self.evaluate(&unary.expr);

        match unary.op.token_type {
            TokenType::Minus => Box::new(
                boxed_right
                    .downcast_mut::<f64>()
                    .map(|&mut v| -v)
                    .expect("Expression should be a valid number"),
            ),
            TokenType::Bang => Box::new(!Interpreter::is_truthy(&boxed_right)),
            t => {
                panic!("Unary operand {t:?} not supported")
            }
        }
    }

    fn visit_binary(&mut self, binary: &'a Binary) -> Self::Return {
        let boxed_left = self.evaluate(&binary.left);
        let boxed_right = self.evaluate(&binary.right);

        match binary.op.token_type {
            TokenType::Minus => Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .expect("Left of - binary should be a valid number")
                    - boxed_right
                        .downcast_ref::<f64>()
                        .expect("Right of - binary should be a valid number"),
            ),
            TokenType::Slash => Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .expect("Left of / binary should be a valid number")
                    / boxed_right
                        .downcast_ref::<f64>()
                        .expect("Right of / binary should be a valid number"),
            ),
            TokenType::Star => Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .expect("Left of * binary should be a valid number")
                    * boxed_right
                        .downcast_ref::<f64>()
                        .expect("Right of * binary should be a valid number"),
            ),
            TokenType::Plus => {
                // + can add together two numbers or two strings
                if boxed_left.is::<f64>() && boxed_right.is::<f64>() {
                    Box::new(
                        boxed_left
                            .downcast_ref::<f64>()
                            .expect("Left of + binary should be a valid number")
                            + boxed_right
                                .downcast_ref::<f64>()
                                .expect("Right of + binary should be a valid number"),
                    )
                } else if boxed_left.is::<String>() && boxed_right.is::<String>() {
                    Box::new(format!(
                        "{}{}",
                        boxed_left
                            .downcast_ref::<String>()
                            .expect("Left of + binary should be a valid string"),
                        boxed_right
                            .downcast_ref::<f64>()
                            .expect("Right of + binary should be a valid string"),
                    ))
                } else {
                    panic!("Binary operand + cannot be evaluated")
                }
            }
            TokenType::Greater => Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .expect("Left of > binary should be a valid number")
                    > boxed_right
                        .downcast_ref::<f64>()
                        .expect("Right of > binary should be a valid number"),
            ),
            TokenType::GreaterEqual => Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .expect("Left of >= binary should be a valid number")
                    >= boxed_right
                        .downcast_ref::<f64>()
                        .expect("Right of >= binary should be a valid number"),
            ),
            TokenType::Less => Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .expect("Left of < binary should be a valid number")
                    < boxed_right
                        .downcast_ref::<f64>()
                        .expect("Right of < binary should be a valid number"),
            ),
            TokenType::LessEqual => Box::new(
                boxed_left
                    .downcast_ref::<f64>()
                    .expect("Left of <= binary should be a valid number")
                    <= boxed_right
                        .downcast_ref::<f64>()
                        .expect("Right of <= binary should be a valid number"),
            ),
            TokenType::EqualEqual => Box::new(is_equal(&*boxed_left, &*boxed_right)),
            TokenType::BangEqual => Box::new(!is_equal(&*boxed_left, &*boxed_right)),
            t => {
                panic!("Binary operand {t:?} not supported")
            }
        }
    }

    fn visit_grouping(&mut self, grouping: &'a Grouping) -> Self::Return {
        self.evaluate(&grouping.expr)
    }

    fn visit_literal(&mut self, literal: &'a Literal) -> Self::Return {
        match literal {
            Literal::Boolean(v) => Box::new(*v),
            Literal::String(v) => Box::new(v.clone()),
            Literal::Nil => Box::new(()),
            Literal::Number(v) => Box::new(*v),
        }
    }
}

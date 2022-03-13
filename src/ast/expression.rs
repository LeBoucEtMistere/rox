use crate::token::Token;

use super::visitor::ExprVisitor;

/// Base structure of the AST
pub enum Expr {
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
}

pub struct Unary {
    pub op: Token,
    pub expr: Box<Expr>,
}

pub struct Binary {
    pub left: Box<Expr>,
    pub op: Token,
    pub right: Box<Expr>,
}
pub struct Grouping {
    pub expr: Box<Expr>,
}

pub struct Literal {
    pub value: Token,
}

impl Expr {
    pub(super) fn accept<T>(&self, visitor: &mut dyn ExprVisitor<Return = T>) -> T {
        match self {
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
        }
    }

    /// Helper function to generate a binary expression instance
    pub fn new_binary(left: Expr, op: Token, right: Expr) -> Self {
        Expr::Binary(Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    /// Helper function to generate a unary expression instance
    pub fn new_unary(op: Token, expr: Expr) -> Self {
        Expr::Unary(Unary {
            op,
            expr: Box::new(expr),
        })
    }

    /// Helper function to generate literal expression instance
    pub fn new_literal(value: Token) -> Self {
        Expr::Literal(Literal { value })
    }

    /// Helper function to generate a grouping expression instance
    pub fn new_grouping(expr: Expr) -> Self {
        Expr::Grouping(Grouping {
            expr: Box::new(expr),
        })
    }
}

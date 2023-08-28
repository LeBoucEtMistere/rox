use super::visitor::ExprVisitor;
use crate::token::Token;

/// Base structure of the AST
pub enum Expr {
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Variable(Variable),
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

#[derive(Debug)]
pub enum Literal {
    Boolean(bool),
    String(String),
    Nil,
    Number(f64),
}

pub struct Variable {
    pub name: Token,
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<Return = T>) -> T {
        match self {
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Variable(variable) => visitor.visit_variable(variable),
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

    /// Helper function to generate boolean literal expression instance
    pub fn new_boolean_literal(v: bool) -> Self {
        Expr::Literal(Literal::Boolean(v))
    }

    /// Helper function to generate boolean literal expression instance
    pub fn new_nil_literal() -> Self {
        Expr::Literal(Literal::Nil)
    }

    /// Helper function to generate boolean literal expression instance
    pub fn new_number_literal(v: f64) -> Self {
        Expr::Literal(Literal::Number(v))
    }

    /// Helper function to generate boolean literal expression instance
    pub fn new_string_literal(v: String) -> Self {
        Expr::Literal(Literal::String(v))
    }

    /// Helper function to generate a grouping expression instance
    pub fn new_grouping(expr: Expr) -> Self {
        Expr::Grouping(Grouping {
            expr: Box::new(expr),
        })
    }

    pub fn new_variable(name: Token) -> Self {
        Expr::Variable(Variable { name })
    }
}

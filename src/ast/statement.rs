use super::{visitor::StatementVisitor, Expr};
use crate::token::Token;

pub enum Statement {
    Expression(ExpressionStatement),
    Print(PrintStatement),
    Variable(VariableStatement),
}

pub struct ExpressionStatement {
    pub expr: Expr,
}

pub struct PrintStatement {
    pub expr: Expr,
}

pub struct VariableStatement {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut dyn StatementVisitor<Return = T>) -> T {
        match self {
            Statement::Expression(v) => visitor.visit_expression(v),
            Statement::Print(v) => visitor.visit_print(v),
            Statement::Variable(v) => visitor.visit_variable(v),
        }
    }
    pub fn new_expression_statement(expr: Expr) -> Self {
        Self::Expression(ExpressionStatement { expr })
    }

    pub fn new_print_statement(expr: Expr) -> Self {
        Self::Print(PrintStatement { expr })
    }

    pub fn new_var_statement(name: Token, initializer: Option<Expr>) -> Self {
        Self::Variable(VariableStatement { name, initializer })
    }
}

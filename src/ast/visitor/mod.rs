// mod ast_pretty_printer;

// pub use ast_pretty_printer::ASTPrettyPrinter;

use super::{
    expression::{Binary, Grouping, Literal, Unary, Variable},
    statement::{ExpressionStatement, PrintStatement, VariableStatement},
};

/// Base trait to define a visitor for the AST
pub trait ExprVisitor {
    type Return;

    /// Visit an unary expression
    fn visit_unary(&mut self, unary: &Unary) -> Self::Return;
    /// Visit a binary expression
    fn visit_binary(&mut self, binary: &Binary) -> Self::Return;
    /// Visit a grouping expression
    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Return;
    /// Visit a literal expression
    fn visit_literal(&mut self, literal: &Literal) -> Self::Return;
    /// Visit a variable expression
    fn visit_variable(&mut self, variable: &Variable) -> Self::Return;
}

pub trait StatementVisitor {
    type Return;

    /// visit a print statement
    fn visit_print(&mut self, statement: &PrintStatement) -> Self::Return;
    /// visit an expression statement
    fn visit_expression(&mut self, expression: &ExpressionStatement) -> Self::Return;
    /// visit a variable statement
    fn visit_variable(&mut self, variable: &VariableStatement) -> Self::Return;
}

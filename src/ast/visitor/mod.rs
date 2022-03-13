mod ast_printer;

pub use ast_printer::ASTPrinter;

use super::expression::{Binary, Grouping, Literal, Unary};

/// Base trait to define a visitor for the AST
pub trait ExprVisitor {
    type Return;

    /// Visit an unary expression
    fn visit_unary(&self, unary: &Unary) -> Self::Return;
    /// Visit a binary expression
    fn visit_binary(&self, binary: &Binary) -> Self::Return;
    /// Visit a grouping expression
    fn visit_grouping(&self, grouping: &Grouping) -> Self::Return;
    /// Visit a literal expression
    fn visit_literal(&self, literal: &Literal) -> Self::Return;
}

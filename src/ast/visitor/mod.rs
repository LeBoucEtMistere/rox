mod ast_pretty_printer;

pub use ast_pretty_printer::ASTPrettyPrinter;

use super::expression::{Binary, Grouping, Literal, Unary};

/// Base trait to define a visitor for the AST
pub trait ExprVisitor<'a> {
    type Return;

    /// Visit an unary expression
    fn visit_unary(&mut self, unary: &'a Unary) -> Self::Return;
    /// Visit a binary expression
    fn visit_binary(&mut self, binary: &'a Binary) -> Self::Return;
    /// Visit a grouping expression
    fn visit_grouping(&mut self, grouping: &'a Grouping) -> Self::Return;
    /// Visit a literal expression
    fn visit_literal(&mut self, literal: &'a Literal) -> Self::Return;
}

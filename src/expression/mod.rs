mod ast_printer;

use crate::token::Token;

enum Expr {
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
}

struct Unary {
    op: Token,
    expr: Box<Expr>,
}

struct Binary {
    left: Box<Expr>,
    op: Token,
    right: Box<Expr>,
}
struct Grouping {
    expr: Box<Expr>,
}

struct Literal {
    value: Token,
}

trait ExprVisitor {
    type Return;

    fn visit_unary(&self, unary: &Unary) -> Self::Return;
    fn visit_binary(&self, binary: &Binary) -> Self::Return;
    fn visit_grouping(&self, grouping: &Grouping) -> Self::Return;
    fn visit_literal(&self, literal: &Literal) -> Self::Return;
}

impl Expr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<Return = T>) -> T {
        match self {
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
        }
    }
}

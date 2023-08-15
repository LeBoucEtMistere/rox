use super::{visitor::StatementVisitor, Expr};

pub enum Statement {
    Expression(ExpressionStatement),
    Print(PrintStatement),
}

pub struct ExpressionStatement {
    pub expr: Expr,
}

pub struct PrintStatement {
    pub expr: Expr,
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut dyn StatementVisitor<Return = T>) -> T {
        match self {
            Statement::Expression(v) => visitor.visit_expression(v),
            Statement::Print(v) => visitor.visit_print(v),
        }
    }
    pub fn new_expression_statement(expr: Expr) -> Self {
        Self::Expression(ExpressionStatement { expr })
    }

    pub fn new_print_statement(expr: Expr) -> Self {
        Self::Print(PrintStatement { expr })
    }
}

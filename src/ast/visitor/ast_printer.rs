use std::ops::Deref;

use super::ExprVisitor;
use crate::ast::expression::{Binary, Expr, Grouping, Literal, Unary};

pub struct ASTPrinter {}

impl ExprVisitor for ASTPrinter {
    type Return = String;

    fn visit_unary(&mut self, unary: &Unary) -> Self::Return {
        self.parenthesize(&unary.op.lexeme, std::slice::from_ref(&unary.expr))
    }

    fn visit_binary(&mut self, binary: &Binary) -> Self::Return {
        self.parenthesize(
            &binary.op.lexeme,
            &[binary.left.as_ref(), binary.right.as_ref()],
        )
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Return {
        self.parenthesize("group", std::slice::from_ref(&grouping.expr))
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Return {
        literal.value.lexeme.to_string()
    }
}

impl ASTPrinter {
    /// Render an AST in a simple String
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    /// Helper function to properly parenthesizes levels of the AST
    fn parenthesize(&mut self, op_name: &str, exprs: &[impl Deref<Target = Expr>]) -> String {
        let mut output = String::new();
        output.push('(');
        output.push_str(op_name);
        for expr in exprs {
            output.push(' ');
            output.push_str(&expr.accept(self))
        }
        output.push(')');

        output
    }
}

#[cfg(test)]
mod test {
    use super::ASTPrinter;
    use crate::{
        ast::expression::Expr,
        token::{Token, TokenType},
    };

    #[test]
    fn basic_test() {
        let expr = Expr::new_binary(
            Expr::new_unary(
                Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".into(),
                    line: 0,
                },
                Expr::new_literal(Token {
                    token_type: TokenType::Number,
                    lexeme: "123".into(),
                    line: 0,
                }),
            ),
            Token {
                token_type: TokenType::Star,
                lexeme: "*".into(),
                line: 0,
            },
            Expr::new_grouping(Expr::new_literal(Token {
                token_type: TokenType::Number,
                lexeme: "45.67".into(),
                line: 0,
            })),
        );

        assert_eq!(ASTPrinter {}.print(&expr), "(* (- 123) (group 45.67))");
    }
}

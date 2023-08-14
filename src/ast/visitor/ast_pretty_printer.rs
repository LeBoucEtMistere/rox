use std::ops::Deref;

use super::ExprVisitor;
use crate::ast::expression::{Binary, Expr, Grouping, Literal, Unary};

pub struct ASTPrettyPrinter {
    indent_lvl: usize,
}

impl ExprVisitor for ASTPrettyPrinter {
    type Return = String;

    fn visit_unary(&mut self, unary: &Unary) -> Self::Return {
        self.format(&unary.op.lexeme, std::slice::from_ref(&unary.expr))
    }

    fn visit_binary(&mut self, binary: &Binary) -> Self::Return {
        self.format(
            &binary.op.lexeme,
            &[binary.left.as_ref(), binary.right.as_ref()],
        )
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Return {
        self.format("group", std::slice::from_ref(&grouping.expr))
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Return {
        let mut output = String::new();
        if self.indent_lvl > 0 {
            output.push_str(&"│  ".repeat(self.indent_lvl - 1));
            output.push_str("└─ ");
        }
        match literal {
            Literal::Boolean(v) => output.push_str(&format!("{v}")),
            Literal::String(v) => output.push_str(v),
            Literal::Nil => output.push_str("nil"),
            Literal::Number(v) => output.push_str(&format!("{v}")),
        }

        output
    }
}

impl ASTPrettyPrinter {
    pub fn new() -> Self {
        ASTPrettyPrinter { indent_lvl: 0 }
    }
    /// Render an AST in a pretty printed fashion String
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    /// Helper function to properly indent levels of the AST
    fn format(&mut self, op_name: &str, children: &[impl Deref<Target = Expr>]) -> String {
        let mut output = String::new();
        if self.indent_lvl > 0 {
            output.push_str(&"│  ".repeat(self.indent_lvl - 1));
            output.push_str("└─ ");
        }

        output.push_str(op_name);
        self.indent_lvl += 1;
        for expr in children {
            output.push('\n');
            output.push_str(&expr.accept(self));
        }
        self.indent_lvl -= 1;

        output
    }
}

#[cfg(test)]
mod test {
    use super::ASTPrettyPrinter;
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
                Expr::new_number_literal(123.0),
            ),
            Token {
                token_type: TokenType::Star,
                lexeme: "*".into(),
                line: 0,
            },
            Expr::new_grouping(Expr::new_number_literal(45.67)),
        );

        assert_eq!(
            ASTPrettyPrinter::new().print(&expr),
            "*\n└─ -\n│  └─ 123\n└─ group\n│  └─ 45.67"
        );
    }
}

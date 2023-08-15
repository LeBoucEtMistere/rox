use std::ops::Deref;

use super::{ExprVisitor, StatementVisitor};
use crate::ast::{
    expression::{Binary, Expr, Grouping, Literal, Unary},
    statement::{ExpressionStatement, PrintStatement},
    Statement,
};

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

impl StatementVisitor for ASTPrettyPrinter {
    type Return = String;

    fn visit_print(&mut self, statement: &PrintStatement) -> Self::Return {
        format! {"print {}", statement.expr.accept(self)}
    }

    fn visit_expression(&mut self, statement: &ExpressionStatement) -> Self::Return {
        statement.expr.accept(self).to_string()
    }
}

impl ASTPrettyPrinter {
    pub fn new() -> Self {
        ASTPrettyPrinter { indent_lvl: 0 }
    }
    /// Render an AST in a pretty printed fashion String
    pub fn print(&mut self, statements: &Vec<Statement>) -> String {
        let mut s = String::new();
        for statement in statements {
            s += &statement.accept(self);
            s += "\n";
        }
        s
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
        ast::{expression::Expr, Statement},
        token::{Token, TokenType},
    };

    #[test]
    fn basic_test() {
        let statements = vec![Statement::new_expression_statement(Expr::new_binary(
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
        ))];

        assert_eq!(
            ASTPrettyPrinter::new().print(&statements),
            "*\n└─ -\n│  └─ 123\n└─ group\n│  └─ 45.67"
        );
    }
}

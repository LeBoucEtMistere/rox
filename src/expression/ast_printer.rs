use std::ops::Deref;

use super::{Binary, Expr, ExprVisitor, Grouping, Literal, Unary};

struct ASTPrinter {}

impl ExprVisitor for ASTPrinter {
    type Return = String;

    fn visit_unary(&self, unary: &Unary) -> Self::Return {
        self.parenthesize(&unary.op.lexeme, std::slice::from_ref(&unary.expr))
    }

    fn visit_binary(&self, binary: &Binary) -> Self::Return {
        self.parenthesize(
            &binary.op.lexeme,
            &[binary.left.as_ref(), binary.right.as_ref()],
        )
    }

    fn visit_grouping(&self, grouping: &Grouping) -> Self::Return {
        self.parenthesize("group", std::slice::from_ref(&grouping.expr))
    }

    fn visit_literal(&self, literal: &Literal) -> Self::Return {
        literal.value.lexeme.to_string()
    }
}

impl ASTPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }
    fn parenthesize(&self, op_name: &str, exprs: &[impl Deref<Target = Expr>]) -> String {
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
    use crate::token::{Token, TokenType};

    use super::super::{Binary, Expr, Grouping, Literal, Unary};
    #[test]
    fn basic_test() {
        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Unary(Unary {
                op: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".into(),
                    line: 0,
                },
                expr: Box::new(Expr::Literal(Literal {
                    value: Token {
                        token_type: TokenType::Number,
                        lexeme: "123".into(),
                        line: 0,
                    },
                })),
            })),
            op: Token {
                token_type: TokenType::Star,
                lexeme: "*".into(),
                line: 0,
            },
            right: Box::new(Expr::Grouping(Grouping {
                expr: Box::new(Expr::Literal(Literal {
                    value: Token {
                        token_type: TokenType::Number,
                        lexeme: "45.67".into(),
                        line: 0,
                    },
                })),
            })),
        });

        assert_eq!(ASTPrinter {}.print(expr), "(* (- 123) (group 45.67))");
    }
}

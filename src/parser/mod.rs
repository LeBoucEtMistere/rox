pub mod error;

use error::ParserError;

use self::error::ParserResults;
use crate::{
    ast::{Expr, Statement},
    token::{Token, TokenType},
};

/// Implements the parsing of tokens obtained from the scanner into an AST,
/// based on the rules of the following grammar:
///
/// program               → declaration* EOF ;
///
/// declaration           → var_decl | statement ;
/// var_decl              → "var" IDENTIFIER ( "=" expression )? ";" ;
/// statement             → expression_statement | print_statement ;
/// expression_statement  → expression ";" ;
/// print_statement       → print expression  ";" ;
///
/// expression            → assignment ;
/// assignment           → IDENTIFIER "=" assignment | equality ;
/// equality              → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison            → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term                  → factor ( ( "-" | "+" ) factor )* ;
/// factor                → unary ( ( "/" | "*" ) unary )* ;
/// unary                 → ( "!" | "-" ) unary | primary ;
/// primary               → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
///                         | IDENTIFIER ;
pub struct Parser {
    /// Holds the list of tokens being parsed
    tokens: Vec<Token>,
    /// Internal state: keep track of the current token index
    current_index: usize,
}

impl Parser {
    /// Builds a parser from a Vec of tokens obtained from the scanner
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_index: 0,
        }
    }

    /// Parse the given tokens into an AST using the rules of the grammer
    pub fn parse(mut self) -> ParserResults<Vec<Statement>> {
        let mut statements = Vec::new();
        let mut errors_encountered: Vec<ParserError> = Vec::new();

        while self.peek().token_type != TokenType::Eof {
            match self.declaration() {
                Ok(s) => statements.push(s),
                Err(e) => errors_encountered.push(e),
            };
        }

        if !errors_encountered.is_empty() {
            Err(errors_encountered)
        } else {
            Ok(statements)
        }
    }

    // Grammar rules

    /// Defines the rule to parse the declaration rule in the grammar:
    /// declaration           → var_decl | statement ;
    fn declaration(&mut self) -> Result<Statement, ParserError> {
        let result = if self.advance_if_token_type_matches(&[TokenType::Var]) {
            self.var_decl()
        } else {
            self.statement()
        };
        result.map_err(|err| {
            // synchronize the internal state to prepare the possible next call to `declaration()`
            self.synchronize();
            err
        })
    }

    /// Defines the rule to parse the declaration rule in the grammar:
    /// var_decl              → "var" IDENTIFIER ( "=" expression )? ";" ;
    fn var_decl(&mut self) -> Result<Statement, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expected variable name".into())?;

        let initializer = if self.advance_if_token_type_matches(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable delcaration".into(),
        )?;

        Ok(Statement::new_var_statement(name, initializer))
    }

    /// Defines the rule to parse the statement rule in the grammar:
    /// statement             → expression_statement | print_statement ;
    fn statement(&mut self) -> Result<Statement, ParserError> {
        if self.advance_if_token_type_matches(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    /// Defines the rule to parse the print_statement rule in the grammar:
    /// print_statement       → print expression  ";" ;
    fn print_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.".into())?;
        Ok(Statement::new_print_statement(expr))
    }

    /// Defines the rule to parse the expression_statement rule in the grammar:
    /// expression_statement  → expression ";" ;
    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.".into())?;
        Ok(Statement::new_expression_statement(expr))
    }

    /// Defines the rule to parse the expression rule in the grammar:
    /// expression     → assignment ;
    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    /// Defines the rule to parse the assignment rule in the grammar:
    /// assignment     → IDENTIFIER "=" assignment | equality ;
    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.equality()?;

        if self.advance_if_token_type_matches(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?; // assignment is right-associative so we call it again here
            if let Expr::Variable(v) = expr {
                return Ok(Expr::new_assign(v.name, value));
            } else {
                return Err(ParserError::new(equals, "Invalid assignment target".into()));
            }
        }

        Ok(expr)
    }

    /// Defines the rule to parse the equality rule in the grammar:
    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;
        while self.advance_if_token_type_matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.remove_previous();
            let right = self.comparison()?;
            expr = Expr::new_binary(expr, op, right);
        }

        Ok(expr)
    }

    /// Defines the rule to parse the comparison rule in the grammar:
    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;
        while self.advance_if_token_type_matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.remove_previous();
            let right = self.term()?;
            expr = Expr::new_binary(expr, op, right);
        }

        Ok(expr)
    }

    /// Defines the rule to parse the term rule in the grammar:
    /// term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;
        while self.advance_if_token_type_matches(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.remove_previous();
            let right = self.factor()?;
            expr = Expr::new_binary(expr, op, right);
        }

        Ok(expr)
    }

    /// Defines the rule to parse the factor rule in the grammar:
    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;
        while self.advance_if_token_type_matches(&[TokenType::Slash, TokenType::Star]) {
            let op = self.remove_previous();
            let right = self.unary()?;
            expr = Expr::new_binary(expr, op, right);
        }

        Ok(expr)
    }

    /// Defines the rule to parse the unary rule in the grammar:
    /// unary          → ( "!" | "-" ) unary
    ///                | primary ;
    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.advance_if_token_type_matches(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.remove_previous();
            let right = self.unary()?;
            return Ok(Expr::new_unary(op, right));
        }

        self.primary()
    }

    /// Defines the rule to parse the primary rule in the grammar:
    /// primary        → NUMBER | STRING | "true" | "false" | "nil"
    ///                | "(" expression ")" | IDENTIFIER ;
    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.advance_if_token_type_matches(&[TokenType::False, TokenType::True]) {
            return Ok(Expr::new_boolean_literal(
                self.remove_previous().token_type == TokenType::True,
            ));
        }
        if self.advance_if_token_type_matches(&[TokenType::Nil]) {
            return Ok(Expr::new_nil_literal());
        }
        if self.advance_if_token_type_matches(&[TokenType::String]) {
            return Ok(Expr::new_string_literal(self.remove_previous().lexeme));
        }
        if self.advance_if_token_type_matches(&[TokenType::Number]) {
            return Ok(Expr::new_number_literal(
                self.remove_previous()
                    .lexeme
                    .parse::<f64>()
                    .expect("Token should contain valid number after scanning is done."),
            ));
        }
        if self.advance_if_token_type_matches(&[TokenType::Identifier]) {
            return Ok(Expr::new_variable(self.previous().clone()));
        }

        if self.advance_if_token_type_matches(std::slice::from_ref(&TokenType::LeftParen)) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.".into())?;
            return Ok(Expr::new_grouping(expr));
        }

        Err(ParserError::new(
            self.peek().clone(),
            "Expected expression".to_owned(),
        ))
    }

    // Helpers

    fn advance_if_token_type_matches(&mut self, token_types: &[TokenType]) -> bool {
        let token_type = self.peek().token_type;
        if token_types.contains(&token_type) {
            self.advance();
            return true;
        }
        false
    }

    #[inline]
    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current_index)
            .expect("current index shouldn't be greater than number of tokens")
    }

    fn consume(&mut self, token_type: TokenType, error_msg: String) -> Result<Token, ParserError> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            Err(ParserError::new(self.peek().clone(), error_msg))
        }
    }

    #[inline]
    fn check(&self, token_type: TokenType) -> bool {
        self.peek().token_type == token_type
    }

    #[inline]
    fn advance(&mut self) -> &Token {
        if self.peek().token_type != TokenType::Eof {
            self.current_index += 1;
        }
        self.previous()
    }

    #[inline]
    fn previous(&self) -> &Token {
        self.tokens.get(self.current_index - 1).unwrap()
    }

    fn remove_previous(&mut self) -> Token {
        self.current_index -= 1;
        self.tokens.remove(self.current_index)
    }

    #[allow(unused)]
    /// Will be used later on once we add statements to the grammar
    fn synchronize(&mut self) {
        self.advance();
        while self.peek().token_type != TokenType::Eof {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::For
                | TokenType::Fun
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => {
                    return;
                }
                _ => (),
            }
            self.advance();
        }
    }
}

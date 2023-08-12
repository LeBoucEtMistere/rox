use crate::{
    ast::Expr,
    token::{Token, TokenType},
};

/// Implements the parsing of tokens obtained from the scanner into an AST,
/// based on the rules of the following grammer:
///
/// expression     → equality ;
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" ) unary
///                | primary ;
/// primary        → NUMBER | STRING | "true" | "false" | "nil"
///                | "(" expression ")" ;
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Builds a parser from a Vec of tokens obtained from the scanner
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parse the given tokens into an AST using the rules of the grammer
    pub fn parse(mut self) -> Expr {
        self.expression()
    }

    // Grammar rules

    /// Defines the rule to parse the expression rule in the grammar:
    /// expression     → equality ;
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    /// Defines the rule to parse the equality rule in the grammar:
    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.advance_if_token_type_matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.remove_previous();
            let right = self.comparison();
            expr = Expr::new_binary(expr, op, right);
        }

        expr
    }

    /// Defines the rule to parse the comparison rule in the grammar:
    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.advance_if_token_type_matches(&[
            TokenType::Greater,
            TokenType::GreateEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.remove_previous();
            let right = self.term();
            expr = Expr::new_binary(expr, op, right);
        }

        expr
    }

    /// Defines the rule to parse the term rule in the grammar:
    /// term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.advance_if_token_type_matches(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.remove_previous();
            let right = self.factor();
            expr = Expr::new_binary(expr, op, right);
        }

        expr
    }

    /// Defines the rule to parse the factor rule in the grammar:
    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.advance_if_token_type_matches(&[TokenType::Slash, TokenType::Star]) {
            let op = self.remove_previous();
            let right = self.unary();
            expr = Expr::new_binary(expr, op, right);
        }

        expr
    }

    /// Defines the rule to parse the unary rule in the grammar:
    /// unary          → ( "!" | "-" ) unary
    ///                | primary ;
    fn unary(&mut self) -> Expr {
        if self.advance_if_token_type_matches(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.remove_previous();
            let right = self.unary();
            return Expr::new_unary(op, right);
        }

        self.primary()
    }

    /// Defines the rule to parse the primary rule in the grammar:
    /// primary        → NUMBER | STRING | "true" | "false" | "nil"
    ///                | "(" expression ")" ;
    fn primary(&mut self) -> Expr {
        if self.advance_if_token_type_matches(&[
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::String,
            TokenType::Number,
        ]) {
            return Expr::new_literal(self.remove_previous());
        }
        if self.advance_if_token_type_matches(std::slice::from_ref(&TokenType::LeftParen)) {
            let expr = self.expression();
            if !self.advance_if_token_type_matches(std::slice::from_ref(&TokenType::RightParen)) {
                panic!("Expect ')' after expression.")
            }
            return Expr::new_grouping(expr);
        }

        unreachable!()
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
            .get(self.current)
            .expect("current index shouldn't be greater than number of tokens")
    }

    fn advance(&mut self) -> &Token {
        if self.peek().token_type != TokenType::Eof {
            self.current += 1;
        }
        self.previous()
    }

    #[inline]
    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn remove_previous(&mut self) -> Token {
        self.current -= 1;
        self.tokens.remove(self.current)
    }
}

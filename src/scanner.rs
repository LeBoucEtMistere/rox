use crate::{
    error::{InternalRoxError, InternalRoxResult},
    token::{Token, TokenType},
};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<InternalRoxError>> {
        let mut errors_encountered: Vec<InternalRoxError> = Vec::new();

        while self.current < self.source.len() {
            self.start = self.current;
            match self.scan_token() {
                Ok(r) => {
                    // if we have a token to add, add it
                    // this can be None for some reasons, for instance finding whitespaces
                    if let Some(token) = r {
                        self.tokens.push(token)
                    }
                }
                Err(e) => errors_encountered.push(e),
            }
        }
        if errors_encountered.is_empty() {
            self.tokens
                .push(Token::new(TokenType::Eof, String::new(), self.line));
            Ok(self.tokens)
        } else {
            Err(errors_encountered)
        }
    }

    fn scan_token(&mut self) -> InternalRoxResult<Option<Token>> {
        match self.advance() {
            '(' => Ok(Some(self.build_simple_token(TokenType::LeftParen))),
            ')' => Ok(Some(self.build_simple_token(TokenType::RightParen))),
            '{' => Ok(Some(self.build_simple_token(TokenType::LeftBrace))),
            '}' => Ok(Some(self.build_simple_token(TokenType::RightBrace))),
            ',' => Ok(Some(self.build_simple_token(TokenType::Comma))),
            '.' => Ok(Some(self.build_simple_token(TokenType::Dot))),
            '-' => Ok(Some(self.build_simple_token(TokenType::Minus))),
            '+' => Ok(Some(self.build_simple_token(TokenType::Plus))),
            ';' => Ok(Some(self.build_simple_token(TokenType::Semicolon))),
            '*' => Ok(Some(self.build_simple_token(TokenType::Star))),
            '!' => Ok(Some({
                let tt = if self.advance_if_equal('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.build_simple_token(tt)
            })),
            '=' => Ok(Some({
                let tt = if self.advance_if_equal('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.build_simple_token(tt)
            })),
            '<' => Ok(Some({
                let tt = if self.advance_if_equal('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.build_simple_token(tt)
            })),
            '>' => Ok(Some({
                let tt = if self.advance_if_equal('=') {
                    TokenType::GreateEqual
                } else {
                    TokenType::Greater
                };
                self.build_simple_token(tt)
            })),
            '/' => Ok(if self.advance_if_equal('/') {
                // A comment goes until the end of the line.

                while let Some(next_c) = self.peek() {
                    if next_c == '\n' {
                        break;
                    }
                    self.advance();
                }
                None
            } else {
                Some(self.build_simple_token(TokenType::Slash))
            }),
            '"' => self.scan_string(),
            ' ' => Ok(None),
            '\r' => Ok(None),
            '\t' => Ok(None),
            '\n' => {
                self.line += 1;
                Ok(None)
            }
            '0'..='9' => self.scan_number(),
            _ => Err(InternalRoxError::SyntaxError {
                line: self.line,
                message: "Unexpected character".into(),
            }),
        }
    }

    fn build_simple_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            self.source
                .chars()
                .skip(self.start)
                .take(self.current - self.start)
                .collect(),
            self.line,
        )
    }

    fn build_complex_token(&self, token_type: TokenType, lexeme: String) -> Token {
        Token::new(token_type, lexeme, self.line)
    }

    fn scan_string(&mut self) -> InternalRoxResult<Option<Token>> {
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            if c == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.peek().is_none() {
            return Err(InternalRoxError::SyntaxError {
                line: self.line,
                message: "Unterminated string.".into(),
            });
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes when building the lexeme in the token.
        Ok(Some(
            self.build_complex_token(
                TokenType::String,
                self.source
                    .chars()
                    .skip(self.start + 1)
                    .take(self.current - self.start - 2)
                    .collect(),
            ),
        ))
    }

    fn scan_number(&mut self) -> InternalRoxResult<Option<Token>> {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        Ok(Some(
            self.build_complex_token(
                TokenType::Number,
                self.source
                    .chars()
                    .skip(self.start)
                    .take(self.current - self.start)
                    .collect(),
            ),
        ))
    }

    /// return the current char in source and advance cursor by one
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    /// only consume the next char if it matches the expected one
    fn advance_if_equal(&mut self, expected: char) -> bool {
        match self.source.chars().nth(self.current) {
            Some(c) => {
                if c != expected {
                    return false;
                }
            }
            None => return false,
        }
        self.current += 1;
        true
    }

    /// peek the current character in the source
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    /// peek the next character in source
    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    /// helper to check if a character is a digit
    #[inline]
    fn is_digit(c: Option<char>) -> bool {
        matches!(c, Some('0'..='9'))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        error::InternalRoxError,
        token::{Token, TokenType},
    };

    use super::Scanner;

    #[test]
    fn test_simple() {
        let s = Scanner::new("()");
        let a = s.scan_tokens().unwrap();
        assert_eq!(
            a,
            vec![
                Token::new(TokenType::LeftParen, "(".into(), 0),
                Token::new(TokenType::RightParen, ")".into(), 0),
                Token::new(TokenType::Eof, "".into(), 0)
            ]
        );
    }
    #[test]
    fn test_complex_operators() {
        let input = "// this is a comment
        (( )){} // grouping stuff
        !*+-/=<> <= ==   // operators
        1234.567098 +23
        42
        /";
        let s = Scanner::new(input);
        let a = s.scan_tokens().unwrap();
        assert_eq!(
            a,
            vec![
                Token::new(TokenType::LeftParen, "(".into(), 1),
                Token::new(TokenType::LeftParen, "(".into(), 1),
                Token::new(TokenType::RightParen, ")".into(), 1),
                Token::new(TokenType::RightParen, ")".into(), 1),
                Token::new(TokenType::LeftBrace, "{".into(), 1),
                Token::new(TokenType::RightBrace, "}".into(), 1),
                Token::new(TokenType::Bang, "!".into(), 2),
                Token::new(TokenType::Star, "*".into(), 2),
                Token::new(TokenType::Plus, "+".into(), 2),
                Token::new(TokenType::Minus, "-".into(), 2),
                Token::new(TokenType::Slash, "/".into(), 2),
                Token::new(TokenType::Equal, "=".into(), 2),
                Token::new(TokenType::Less, "<".into(), 2),
                Token::new(TokenType::Greater, ">".into(), 2),
                Token::new(TokenType::LessEqual, "<=".into(), 2),
                Token::new(TokenType::EqualEqual, "==".into(), 2),
                Token::new(TokenType::Number, "1234.567098".into(), 3),
                Token::new(TokenType::Plus, "+".into(), 3),
                Token::new(TokenType::Number, "23".into(), 3),
                Token::new(TokenType::Number, "42".into(), 4),
                Token::new(TokenType::Slash, "/".into(), 5),
                Token::new(TokenType::Eof, "".into(), 5),
            ]
        );
    }
    #[test]
    fn test_errors_on_unknown() {
        let s = Scanner::new("@#(");
        let a = s.scan_tokens().unwrap_err();
        assert_eq!(a.len(), 2);
        for e in a {
            assert!(matches!(e, InternalRoxError::SyntaxError { line, message }
                if line == 0 && &message == "Unexpected character"));
        }
    }
}

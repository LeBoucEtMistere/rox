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
                    if let Some(token_type) = r {
                        self.add_token(token_type)
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

    fn scan_token(&mut self) -> InternalRoxResult<Option<TokenType>> {
        match self.advance() {
            '(' => Ok(Some(TokenType::LeftParen)),
            ')' => Ok(Some(TokenType::RightParen)),
            '{' => Ok(Some(TokenType::LeftBrace)),
            '}' => Ok(Some(TokenType::RightBrace)),
            ',' => Ok(Some(TokenType::Comma)),
            '.' => Ok(Some(TokenType::Dot)),
            '-' => Ok(Some(TokenType::Minus)),
            '+' => Ok(Some(TokenType::Plus)),
            ';' => Ok(Some(TokenType::Semicolon)),
            '*' => Ok(Some(TokenType::Star)),
            '!' => Ok(Some(if self.advance_if_equal('=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            })),
            '=' => Ok(Some(if self.advance_if_equal('=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            })),
            '<' => Ok(Some(if self.advance_if_equal('=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            })),
            '>' => Ok(Some(if self.advance_if_equal('=') {
                TokenType::GreateEqual
            } else {
                TokenType::Greater
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
                Some(TokenType::Slash)
            }),
            ' ' => Ok(None),
            '\r' => Ok(None),
            '\t' => Ok(None),
            '\n' => {
                self.line += 1;
                Ok(None)
            }
            _ => Err(InternalRoxError::SyntaxError {
                line: self.line,
                message: "Unexpected character".into(),
            }),
        }
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

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            self.source
                .chars()
                .skip(self.start)
                .take(self.current - self.start)
                .collect(),
            self.line,
        ))
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
        ";
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
                Token::new(TokenType::Eof, "".into(), 3),
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

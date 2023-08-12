use phf::phf_map;

use crate::{
    error::{InternalRoxError, InternalRoxResult},
    token::{Token, TokenType},
};

/// Perfect HashMap mapping string keywords to their token type
static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "while" => TokenType::While,
    "var" => TokenType::Var,
};

/// Scanner is responsible from scanning the lexemes into a list of Tokens. This is the first step
/// of the interpreter
pub struct Scanner<'a> {
    /// holds a reference to the source buffer containing the lexemes to scan
    source_buffer: &'a str,
    /// internal state: holds the built tokens
    tokens: Vec<Token>,

    /// internal state: start index in the source of the token being scanned
    start_index: usize,
    /// internal state: index in the source of the lexeme being scanned
    current_index: usize,
    /// internal state: index of the line being scanned
    line_index: usize,
}

impl<'a> Scanner<'a> {
    /// Create a new Scanner object from a reference to a source buffer
    pub fn new(source_buffer: &'a str) -> Self {
        Self {
            source_buffer,
            tokens: vec![],
            start_index: 0,
            current_index: 0,
            line_index: 0,
        }
    }

    /// Main entry point of the scanner logic. Processes the passed lexemes to build a list of
    /// tokens out of it.
    ///
    /// If any errors are encountered during the scanning process, returns them here.
    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<InternalRoxError>> {
        let mut errors_encountered: Vec<InternalRoxError> = Vec::new();

        while self.current_index < self.source_buffer.len() {
            // starting scanning for a new token, reset the start index
            self.start_index = self.current_index;
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
                .push(Token::new(TokenType::Eof, String::new(), self.line_index));
            Ok(self.tokens)
        } else {
            Err(errors_encountered)
        }
    }

    /// Method responsible for the actual scanning of a token
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
                self.line_index += 1;
                Ok(None)
            }
            '0'..='9' => self.scan_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(),
            // TODO: Improve error handling
            _ => Err(InternalRoxError::SyntaxError {
                line: self.line_index,
                message: "Unexpected character".into(),
            }),
        }
    }

    /// Build a simple token representing the source_buffer lexemes in the interval
    /// `[self.start_index..self.current_index]`
    fn build_simple_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            self.source_buffer[self.start_index..self.current_index].to_owned(),
            self.line_index,
        )
    }

    /// Build a complex token out of a specified lexeme string
    fn build_complex_token(&self, token_type: TokenType, lexeme: String) -> Token {
        Token::new(token_type, lexeme, self.line_index)
    }

    /// Scan the internal buffer from the current token until a string ending delimiter lexeme is
    /// found
    fn scan_string(&mut self) -> InternalRoxResult<Option<Token>> {
        while let Some(c) = self.peek() {
            if c == '"' {
                // delimiter is found, end the string, but don't advance yet, this will be done
                // below
                break;
            }
            if c == '\n' {
                // don't forget to advance the line index when scanning multi-line strings
                self.line_index += 1
            };
            self.advance();
        }

        if self.peek().is_none() {
            return Err(InternalRoxError::SyntaxError {
                line: self.line_index,
                message: "Unterminated string.".into(),
            });
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes when building the lexeme in the token.
        Ok(Some(self.build_complex_token(
            TokenType::String,
            // don't forget to account for the " delimiters on both sides when extracting the
            // lexeme string
            self.source_buffer[self.start_index + 1..self.current_index - 1].to_owned(),
        )))
    }

    /// Scan the internal buffer from the current token until it finishes scanning a valid number
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

        Ok(Some(self.build_complex_token(
            TokenType::Number,
            self.source_buffer[self.start_index..self.current_index].to_owned(),
        )))
    }

    /// Scan the internal buffer from the current token to find a valid identifier / keyword
    fn scan_identifier(&mut self) -> InternalRoxResult<Option<Token>> {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = &self.source_buffer[self.start_index..self.current_index];

        Ok(Some(if let Some(token_type) = KEYWORDS.get(text) {
            self.build_complex_token(*token_type, text.to_owned())
        } else {
            self.build_complex_token(TokenType::Identifier, text.to_owned())
        }))
    }

    /// return the current char in source and advance cursor by one
    fn advance(&mut self) -> char {
        self.current_index += 1;
        self.source_buffer
            .chars()
            .nth(self.current_index - 1)
            .unwrap()
    }

    /// only consume the next char if it matches the expected one
    fn advance_if_equal(&mut self, expected: char) -> bool {
        match self.source_buffer.chars().nth(self.current_index) {
            Some(c) => {
                if c != expected {
                    return false;
                }
            }
            None => return false,
        }
        self.current_index += 1;
        true
    }

    /// peek the current character in the source
    fn peek(&self) -> Option<char> {
        self.source_buffer.chars().nth(self.current_index)
    }

    /// peek the next character in source
    fn peek_next(&self) -> Option<char> {
        self.source_buffer.chars().nth(self.current_index + 1)
    }

    /// helper to check if a character is a digit
    #[inline]
    fn is_digit(c: Option<char>) -> bool {
        matches!(c, Some('0'..='9'))
    }

    /// helper to check if a character is alpha
    #[inline]
    fn is_alpha(c: Option<char>) -> bool {
        matches!(c, Some('a'..='z' | 'A'..='Z' | '_'))
    }

    /// helper to check if a character is alphanumeric
    #[inline]
    fn is_alphanumeric(c: Option<char>) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }
}

#[cfg(test)]
mod test {
    use super::Scanner;
    use crate::{
        error::InternalRoxError,
        token::{Token, TokenType},
    };

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
        \"aaaaaa\"
        or
        baba_is_you
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
                Token::new(TokenType::String, "aaaaaa".into(), 5),
                Token::new(TokenType::Or, "or".into(), 6),
                Token::new(TokenType::Identifier, "baba_is_you".into(), 7),
                Token::new(TokenType::Slash, "/".into(), 8),
                Token::new(TokenType::Eof, "".into(), 8),
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

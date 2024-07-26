use crate::token::TokenType::{
    BANG, BANG_EQUAL, COMMA, DOT, EOF, EQUAL, EQUAL_EQUAL, GREATER, GREATER_EQUAL, LEFT_BRACE,
    LEFT_PAREN, LESS, LESS_EQUAL, MINUS, PLUS, RIGHT_BRACE, RIGHT_PAREN, SEMICOLON, SLASH, STAR,
    STRING,
};
use crate::token::{Token, TokenType};

pub(crate) struct Scanner<'a> {
    start: usize,
    current: usize,
    line: usize,

    source: &'a [u8],
    tokens: Vec<Token<'a>>,
    has_error: bool,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a [u8]) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> (&'a Vec<Token>, bool) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens
            .push(Token::new(EOF, "".as_bytes(), "null", self.line));

        (&self.tokens, self.has_error)
    }

    fn advance(&mut self) -> u8 {
        let char = self.source[self.current];
        self.current += 1;
        char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, "null")
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: &'a str) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    fn next_match(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.source[self.current]
    }

    fn add_string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.", "".into());
            return;
        }

        self.advance();

        self.add_token_with_literal(
            STRING,
            std::str::from_utf8(&self.source[self.start + 1..self.current - 1]).unwrap(),
        );
    }

    fn scan_token(&mut self) {
        match self.advance() {
            b'(' => self.add_token(LEFT_PAREN),
            b')' => self.add_token(RIGHT_PAREN),
            b'{' => self.add_token(LEFT_BRACE),
            b'}' => self.add_token(RIGHT_BRACE),
            b',' => self.add_token(COMMA),
            b'.' => self.add_token(DOT),
            b'-' => self.add_token(MINUS),
            b'+' => self.add_token(PLUS),
            b';' => self.add_token(SEMICOLON),
            b'*' => self.add_token(STAR),
            b'!' => {
                let token_type = if self.next_match(b'=') {
                    BANG_EQUAL
                } else {
                    BANG
                };
                self.add_token(token_type);
            }
            b'=' => {
                let token_type = if self.next_match(b'=') {
                    EQUAL_EQUAL
                } else {
                    EQUAL
                };
                self.add_token(token_type);
            }
            b'<' => {
                let token_type = if self.next_match(b'=') {
                    LESS_EQUAL
                } else {
                    LESS
                };
                self.add_token(token_type);
            }
            b'>' => {
                let token_type = if self.next_match(b'=') {
                    GREATER_EQUAL
                } else {
                    GREATER
                };
                self.add_token(token_type);
            }
            b'/' => {
                if self.next_match(b'/') {
                    while !self.is_at_end() && self.peek() != b'\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH)
                };
            }
            b' ' | b'\t' | b'\r' => {}
            b'\n' => self.line += 1,
            b'"' => self.add_string(),
            ch => self.error(self.line, "Unexpected character: ", (ch as char).into()),
        }
    }

    fn error(&mut self, line: usize, _where: &'static str, message: String) {
        self.has_error = true;
        eprintln!("[line {}] Error: {}{}", line, _where, message);
    }
}

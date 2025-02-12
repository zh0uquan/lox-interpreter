use crate::token::TokenType::{
    BANG, BANG_EQUAL, COMMA, DOT, EOF, EQUAL, EQUAL_EQUAL, GREATER, GREATER_EQUAL,
    IDENTIFIER, LEFT_BRACE, LEFT_PAREN, LESS, LESS_EQUAL, MINUS, NUMBER, PLUS,
    RIGHT_BRACE, RIGHT_PAREN, SEMICOLON, SLASH, STAR, STRING,
};
use crate::token::{try_get_keyword, Token, TokenType};
use crate::Lox;

pub(crate) struct Scanner<'a, 'b>
where
    'b: 'a,
{
    start: usize,
    current: usize,
    line: usize,

    source: &'a [u8],
    tokens: Vec<Token<'a>>,
    lox: &'b Lox,
}

impl<'a, 'b> Scanner<'a, 'b> {
    pub(crate) fn new(source: &'a [u8], lox: &'b Lox) -> Self {
        Scanner {
            source,
            lox,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> &'a Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens
            .push(Token::new(EOF, "".as_bytes(), "null".into(), self.line));

        &self.tokens
    }

    fn advance(&mut self) -> u8 {
        let char = self.source[self.current];
        self.current += 1;
        char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, "null".into())
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: String) {
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

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return b'\0';
        }
        self.source[self.current + 1]
    }

    fn add_string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.lox
                .report(self.line, "Unterminated string.", "".into());
            return;
        }

        self.advance();

        self.add_token_with_literal(
            STRING,
            std::str::from_utf8(&self.source[self.start + 1..self.current - 1])
                .unwrap()
                .into(),
        )
    }

    fn add_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let str_repr =
            std::str::from_utf8(&self.source[self.start..self.current]).unwrap();
        let double = str_repr.parse::<f32>().unwrap();
        let double = if double.fract() == 0.0 {
            format!("{:.1}", double)
        } else {
            format!("{}", double)
        };
        self.add_token_with_literal(NUMBER, double)
    }

    fn add_identifier_or_reserved_words(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == b'_' {
            self.advance();
        }

        let str = &std::str::from_utf8(&self.source[self.start..self.current]).unwrap();
        match try_get_keyword(str) {
            None => self.add_token_with_literal(IDENTIFIER, String::from(*str)),
            Some(token) => self.add_token(token),
        }
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
            b'0'..=b'9' => self.add_number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.add_identifier_or_reserved_words(),
            ch => {
                self.lox
                    .report(self.line, "Unexpected character: ", (ch as char).into())
            }
        }
    }
}

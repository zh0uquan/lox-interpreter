use crate::token::TokenType::{
    COMMA, DOT, EOF, LEFT_BRACE, LEFT_PAREN, MINUS, PLUS, RIGHT_BRACE, RIGHT_PAREN, SEMICOLON, STAR,
};
use crate::token::{Token, TokenType};

pub(crate) struct Scanner<'a> {
    start: usize,
    current: usize,
    line: usize,

    source: &'a [u8],
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a [u8]) -> Scanner {
        Scanner {
            source,
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
            .push(Token::new(EOF, "".as_bytes(), "null", self.line));

        &self.tokens
    }

    fn advance(&mut self) -> u8 {
        let char = self.source[self.current];
        self.current += 1;
        char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, "null")
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: &'static str) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
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
            ch => self.error(self.line, "Unexpected character", (ch as char).into()),
        }
    }

    fn error(&mut self, line: usize, _where: &'static str, message: String) {
        eprintln!("[line {}] Error: {}: {}", line, _where, message);
    }
}

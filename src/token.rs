use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types, dead_code)]
pub enum TokenType {
    // Single-character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    // End of file
    EOF,
}

const fn create_keywords() -> [(&'static str, TokenType); 16] {
    [
        ("and", TokenType::AND),
        ("class", TokenType::CLASS),
        ("else", TokenType::ELSE),
        ("false", TokenType::FALSE),
        ("for", TokenType::FOR),
        ("fun", TokenType::FUN),
        ("if", TokenType::IF),
        ("nil", TokenType::NIL),
        ("or", TokenType::OR),
        ("print", TokenType::PRINT),
        ("return", TokenType::RETURN),
        ("super", TokenType::SUPER),
        ("this", TokenType::THIS),
        ("true", TokenType::TRUE),
        ("var", TokenType::VAR),
        ("while", TokenType::WHILE),
    ]
}

const KEYWORDS: [(&str, TokenType); 16] = create_keywords();

pub fn try_get_keyword(keyword: &str) -> Option<TokenType> {
    KEYWORDS
        .into_iter()
        .find(|&(key, _)| key == keyword)
        .map(|(_, token_type)| token_type)
}

#[allow(dead_code)]
pub struct Token<'a> {
    pub(crate) token_type: TokenType,
    lexeme: &'a [u8],
    pub(crate) literal: String,
    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a [u8], literal: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lexeme_str = String::from_utf8_lossy(self.lexeme);
        write!(f, "{:?} {} {}", self.token_type, lexeme_str, self.literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() {
        let t = Token::new(TokenType::LEFT_PAREN, &[40], "null".into(), 0);

        println!("{}", t);
    }
}

use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};

use crate::Lox;
use crate::parser::Declaration::VarDecl;
use crate::parser::Expr::{Binary, Grouping, Literal, Unary};
use crate::token::{Token, TokenType};
use crate::token::TokenType::{BANG, BANG_EQUAL, EOF, EQUAL, EQUAL_EQUAL, FALSE, GREATER, GREATER_EQUAL, IDENTIFIER, LEFT_PAREN, LESS, LESS_EQUAL, MINUS, NIL, NUMBER, PLUS, PRINT, RIGHT_PAREN, SEMICOLON, SLASH, STAR, STRING, TRUE, VAR};

pub enum Declaration<'a> {
    VarDecl(Expr<'a>),
    Statement(Statement<'a>),
}

impl<'a> Display for Declaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VarDecl(expr) => write!(f, "{};", expr),
            Declaration::Statement(expr) => write!(f, "{}", expr),
        }
    }
}


pub enum Statement<'a> {
    ExprStmt(Expr<'a>),
    PrintStmt(Expr<'a>),
}

impl<'a> Display for Statement<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::ExprStmt(expr) => write!(f, "{};", expr),
            Statement::PrintStmt(expr) => write!(f, "print {};", expr)
        }
    }
}


pub enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: &'a Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Literal {
        value: Object,
    },
    Unary {
        operator: &'a Token<'a>,
        right: Box<Expr<'a>>,
    },
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Binary {
                left,
                operator,
                right,
            } => {
                write!(
                    f,
                    "({} {} {})",
                    String::from_utf8_lossy(operator.lexeme),
                    left,
                    right
                )
            }
            Grouping { expression } => {
                write!(f, "(group {})", expression)
            }
            Literal { value } => {
                write!(f, "{}", value)
            }
            Unary { operator, right } => {
                write!(
                    f,
                    "({} {})",
                    String::from_utf8_lossy(operator.lexeme),
                    right
                )
            }
        }
    }
}

pub enum Object {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
    Identifier(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "nil"),
            Object::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.1}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Object::String(s) => write!(f, "{}", s),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Identifier(s) => write!(f, "{}", s)
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(n) => {
                write!(f, "{}", n)
            }
            _ => write!(f, "{}", self)
        }
    }
}

pub(crate) struct Parser<'a, 'b> {
    tokens: &'a Vec<Token<'a>>,
    current: RefCell<usize>,
    lox: &'b Lox,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub(crate) fn new(tokens: &'a Vec<Token>, lox: &'b Lox) -> Self {
        Parser {
            tokens,
            current: RefCell::new(0),
            lox,
        }
    }

    fn is_at_end(&self) -> bool {
        if self.tokens[*self.current.borrow()].token_type == EOF {
            return true;
        }
        false
    }

    fn advance(&self) -> &'a Token<'a> {
        if !self.is_at_end() {
            *self.current.borrow_mut() += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn peek(&self) -> &'a Token<'a> {
        &self.tokens[*self.current.borrow()]
    }

    fn previous(&self) -> &'a Token<'a> {
        &self.tokens[*self.current.borrow() - 1]
    }

    fn consume(&self, token_type: TokenType, message: String) {
        if self.check(token_type) {
            self.advance();
            return;
        }
        self.lox.error(self.peek(), message)
    }

    pub(crate) fn parse(&self) -> Vec<Declaration> {
        let mut decls = vec![];
        while !self.is_at_end() {
            decls.push(self.declaration());
        }
        decls
    }

    fn declaration(&self) -> Declaration {
        if self.match_token(&[VAR]) {
            return VarDecl(self.vardecl());
        }
        return Declaration::Statement(self.statement());
    }

    fn vardecl(&self) -> Expr {
        let var_operator = self.previous();
        let primary = self.primary();
        return if !self.match_token(&[EQUAL]) {
            self.consume(SEMICOLON, "Error: missing semicolon at end".into());
            Unary {
                operator: var_operator,
                right: Box::new(primary),
            }
        } else {
            let operator = self.previous();
            let expr = self.expression();
            self.consume(SEMICOLON, "Error: missing semicolon at end".into());
            Unary {
                operator: var_operator,
                right: Box::new(
                    Binary {
                        left: Box::new(primary),
                        operator,
                        right: Box::new(expr),
                    }
                ),
            }
        };
    }

    fn statement(&self) -> Statement {
        if self.match_token(&[PRINT]) {
            let expr = self.expression();
            self.consume(SEMICOLON, "Error: missing semicolon at end".into());
            return Statement::PrintStmt(expr);
        }
        let expr = self.expression();
        self.consume(SEMICOLON, "Error: missing semicolon at end".into());
        Statement::ExprStmt(expr)
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token(&[BANG_EQUAL, EQUAL_EQUAL]) {
            expr = Binary {
                left: Box::new(expr),
                operator: self.previous(),
                right: Box::new(self.comparison()),
            }
        }
        expr
    }

    fn comparison(&self) -> Expr {
        let mut expr = self.term();
        while self.match_token(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            expr = Binary {
                left: Box::new(expr),
                operator: self.previous(),
                right: Box::new(self.term()),
            }
        }
        expr
    }

    fn term(&self) -> Expr {
        let mut expr = self.factor();
        while self.match_token(&[MINUS, PLUS]) {
            expr = Binary {
                left: Box::new(expr),
                operator: self.previous(),
                right: Box::new(self.factor()),
            }
        }
        expr
    }

    fn factor(&self) -> Expr {
        let mut expr = self.unary();
        while self.match_token(&[SLASH, STAR]) {
            expr = Binary {
                left: Box::new(expr),
                operator: self.previous(),
                right: Box::new(self.unary()),
            }
        }
        expr
    }

    fn unary(&self) -> Expr {
        if self.match_token(&[BANG, MINUS]) {
            return Unary {
                operator: self.previous(),
                right: Box::new(self.unary()),
            };
        }
        self.primary()
    }

    fn match_token(&self, token_types: &[TokenType]) -> bool {
        for &token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn primary(&self) -> Expr {
        if self.match_token(&[STRING]) {
            return Literal {
                value: Object::String(self.previous().literal.clone()),
            };
        }

        if self.match_token(&[NUMBER]) {
            return Literal {
                value: Object::Number(self.previous().literal.parse::<f32>().unwrap()),
            };
        }

        if self.match_token(&[TRUE]) {
            return Literal {
                value: Object::Boolean(true),
            };
        }

        if self.match_token(&[FALSE]) {
            return Literal {
                value: Object::Boolean(false),
            };
        }

        if self.match_token(&[NIL]) {
            return Literal { value: Object::Nil };
        }

        if self.match_token(&[IDENTIFIER]) {
            return Literal { value: Object::Identifier(self.previous().literal.clone()) };
        }

        if self.match_token(&[LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(RIGHT_PAREN, "Error: Unmatched parentheses.".into());
            return Grouping {
                expression: Box::new(expr),
            };
        }
        std::process::exit(65);
    }
}

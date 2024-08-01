use crate::token::Token;
use crate::token::TokenType::{EOF, FALSE, NIL, NUMBER, STRING, TRUE};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
pub enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Literal {
        value: Object,
    },
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator, left, right)
            }
            Expr::Grouping { expression } => {
                write!(f, "(group {})", expression)
            }
            Expr::Literal { value } => {
                write!(f, "{}", value)
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator, right)
            }
        }
    }
}
pub enum Object {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
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
            },
            Object::String(s) => write!(f, "{}", s),
            Object::Boolean(b) => write!(f, "{}", b),
        }
    }
}

pub(crate) struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    current: RefCell<usize>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens,
            current: RefCell::new(0),
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

    fn previous(&self) -> &'a Token<'a> {
        &self.tokens[*self.current.borrow() - 1]
    }

    pub(crate) fn parse(&mut self) -> Expr {
        let literal = self.advance();
        match literal.token_type {
            STRING => Expr::Literal {
                value: Object::String(literal.literal.clone()),
            },
            NUMBER => Expr::Literal {
                value: Object::Number(literal.literal.parse::<f32>().unwrap()),
            },
            TRUE => Expr::Literal {
                value: Object::Boolean(true),
            },
            FALSE => Expr::Literal {
                value: Object::Boolean(false),
            },
            NIL => Expr::Literal { value: Object::Nil },
            _ => unimplemented!(),
        }
    }
}

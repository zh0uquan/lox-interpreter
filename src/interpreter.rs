use std::fmt::{Display, Formatter};
use crate::parser::{Expr, Object};
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
    operator: TokenType,
}

impl RuntimeError {
    pub fn new(message: String, operator: TokenType) -> Self {
        RuntimeError { message, operator }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

trait Visitor {
    fn visit_unary(&self, operator: &Token, right: Box<Expr>) -> Result<Object, RuntimeError>;
    fn visit_expr(&self, expr: Expr) -> Result<Object, RuntimeError>;
    fn visit_binary(&self, operator: &Token, left: Box<Expr>, right: Box<Expr>) -> Result<Object, RuntimeError>;
    fn visit_grouping(&self, expr: Box<Expr>) -> Result<Object, RuntimeError>;
}

pub(crate) struct Interpreter;

impl Interpreter {
    pub(crate) fn new() -> Self {
        Interpreter {}
    }

    pub(crate) fn interpret(&self, expr: Expr) -> Result<Expr, RuntimeError> {
        match expr {
            Expr::Literal { value } => Ok(Expr::Literal { value }),
            Expr::Unary { operator, right } => {
                let value = self.visit_unary(&operator, right)?;
                Ok(Expr::Literal { value })
            },
            Expr::Binary { operator, left, right } => {
                let value = self.visit_binary(&operator, left, right)?;
                Ok(Expr::Literal { value })
            },
            Expr::Grouping { expression } => {
                let value = self.visit_grouping(expression)?;
                Ok(Expr::Literal { value })
            }
        }
    }

    fn ensure_literal<'a, 'b>(&'b self, mut expr: Box<Expr<'a>>) -> Result<Object, RuntimeError>
        where
            'b: 'a,
    {
        while !matches!(*expr, Expr::Literal { .. }) {
            expr = Box::new(self.interpret(*expr)?);
        }

        if let Expr::Literal { value } = *expr {
            Ok(value)
        } else {
            unreachable!() // We ensured it's a Literal in the loop
        }
    }
}

impl Visitor for Interpreter {
    fn visit_unary(&self, operator: &Token, right: Box<Expr>) -> Result<Object, RuntimeError> {
        let right_value = self.ensure_literal(right)?;
        match operator.token_type {
            TokenType::BANG => match right_value {
                Object::Boolean(b) => Ok(Object::Boolean(!b)),
                Object::Number(_) => Ok(Object::Boolean(false)),
                Object::Nil => Ok(Object::Boolean(true)),
                _ => Err(RuntimeError::new("Operand must be a boolean or number.".to_string(), operator.token_type)),
            },
            TokenType::MINUS => match right_value {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => Err(RuntimeError::new("Operand must be a number.".to_string(), operator.token_type)),
            },
            _ => Err(RuntimeError::new("Invalid unary operator.".to_string(),  operator.token_type)),
        }
    }

    fn visit_expr(&self, expr: Expr) -> Result<Object, RuntimeError> {
        if let Expr::Literal { value } = expr {
            Ok(value)
        } else {
            Err(RuntimeError::new("Expected literal expression.".to_string(), Token::default()))
        }
    }

    fn visit_binary(&self, operator: &Token, left: Box<Expr>, right: Box<Expr>) -> Result<Object, RuntimeError> {
        let left_value = self.ensure_literal(left)?;
        let right_value = self.ensure_literal(right)?;

        match (left_value, right_value) {
            (Object::Number(left), Object::Number(right)) => match operator.token_type {
                TokenType::PLUS => Ok(Object::Number(left + right)),
                TokenType::MINUS => Ok(Object::Number(left - right)),
                TokenType::STAR => Ok(Object::Number(left * right)),
                TokenType::SLASH => {
                    if right == 0.0 {
                        Err(RuntimeError::new("Division by zero.".to_string(), operator.token_type))
                    } else {
                        Ok(Object::Number(left / right))
                    }
                },
                TokenType::LESS_EQUAL => Ok(Object::Boolean(left <= right)),
                TokenType::LESS => Ok(Object::Boolean(left < right)),
                TokenType::EQUAL_EQUAL => Ok(Object::Boolean(left == right)),
                TokenType::BANG_EQUAL => Ok(Object::Boolean(left != right)),
                TokenType::GREATER_EQUAL => Ok(Object::Boolean(left >= right)),
                TokenType::GREATER => Ok(Object::Boolean(left > right)),
                _ => Err(RuntimeError::new("Invalid binary operator for numbers.".to_string(), operator.token_type)),
            },
            (Object::String(left), Object::String(right)) => match operator.token_type {
                TokenType::PLUS => Ok(Object::String(left + right.as_str())),
                TokenType::EQUAL_EQUAL => Ok(Object::Boolean(left == right)),
                TokenType::BANG_EQUAL => Ok(Object::Boolean(left != right)),
                _ => Err(RuntimeError::new("Invalid binary operator for strings.".to_string(), operator.token_type)),
            },
            (_, _) if matches!(operator.token_type, TokenType::EQUAL_EQUAL) => Ok(Object::Boolean(false)),
            _ => Err(RuntimeError::new("Invalid operands for binary operator.".to_string(),  operator.token_type)),
        }
    }

    fn visit_grouping(&self, expr: Box<Expr>) -> Result<Object, RuntimeError> {
        self.ensure_literal(expr)
    }
}
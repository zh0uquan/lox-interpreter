use std::cell::RefCell;
use std::env::var;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::enviornment::Environment;
use crate::parser::{Declaration, Expr, Object, Statement};
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
    fn visit_unary(
        &self,
        operator: &Token,
        right: Box<Expr>,
    ) -> Result<Object, RuntimeError>;
    fn visit_expr(&self, expr: Box<Expr>) -> Result<Object, RuntimeError>;
    fn visit_binary(
        &self,
        operator: &Token,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Result<Object, RuntimeError>;
    fn visit_grouping(&self, expr: Box<Expr>) -> Result<Object, RuntimeError>;
    fn visit_print_stmt(&self, expr: Box<Expr>) -> Result<Expr, RuntimeError>;
    fn visit_stmt(&self, stmts: Statement) -> Result<Expr, RuntimeError>;
    fn visit_var_decl(&self, decl: Box<Expr>) -> Result<Expr, RuntimeError>;
}

pub(crate) struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub(crate) fn interpret(
        &self,
        declarations: Vec<Declaration>,
    ) -> Result<Vec<Expr>, RuntimeError> {
        declarations
            .into_iter()
            .map(|decl| match decl {
                Declaration::VarDecl(decl) => self.visit_var_decl(Box::new(decl)),
                Declaration::Statement(stmt) => self.visit_stmt(stmt)
            })
            .collect()
    }


    fn ensure_literal<'a, 'b>(
        &'b self,
        mut expr: Box<Expr<'a>>,
    ) -> Result<Object, RuntimeError>
        where
            'b: 'a,
    {
        while !matches!(*expr, Expr::Literal { .. }) {
            expr = Box::new(self.visit_print_stmt(expr)?);
        }

        if let Expr::Literal { value } = *expr {
            Ok(value)
        } else {
            unreachable!() // We ensured it's a Literal in the loop
        }
    }
}

impl Visitor for Interpreter {
    fn visit_unary(
        &self,
        operator: &Token,
        right: Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        let right_value = self.ensure_literal(right)?;
        match operator.token_type {
            TokenType::BANG => match right_value {
                Object::Boolean(b) => Ok(Object::Boolean(!b)),
                Object::Number(_) => Ok(Object::Boolean(false)),
                Object::Nil => Ok(Object::Boolean(true)),
                _ => Err(RuntimeError::new(
                    "Operand must be a boolean or number.".to_string(),
                    operator.token_type,
                )),
            },
            TokenType::MINUS => match right_value {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => Err(RuntimeError::new(
                    "Operand must be a number.".to_string(),
                    operator.token_type,
                )),
            },
            _ => Err(RuntimeError::new(
                "Invalid unary operator.".to_string(),
                operator.token_type,
            )),
        }
    }
    fn visit_expr(&self, expr: Box<Expr>) -> Result<Object, RuntimeError> {
        if let Expr::Literal { value } = *expr {
            Ok(value)
        } else {
            Err(RuntimeError::new(
                "Expected literal expression.".to_string(),
                Token::default(),
            ))
        }
    }

    fn visit_binary(
        &self,
        operator: &Token,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        let left_value = self.ensure_literal(left)?;
        let right_value = self.ensure_literal(right)?;

        match (left_value, right_value) {
            (Object::Number(left), Object::Number(right)) => match operator.token_type {
                TokenType::PLUS => Ok(Object::Number(left + right)),
                TokenType::MINUS => Ok(Object::Number(left - right)),
                TokenType::STAR => Ok(Object::Number(left * right)),
                TokenType::SLASH => {
                    if right == 0.0 {
                        Err(RuntimeError::new(
                            "Division by zero.".to_string(),
                            operator.token_type,
                        ))
                    } else {
                        Ok(Object::Number(left / right))
                    }
                }
                TokenType::LESS_EQUAL => Ok(Object::Boolean(left <= right)),
                TokenType::LESS => Ok(Object::Boolean(left < right)),
                TokenType::EQUAL_EQUAL => Ok(Object::Boolean(left == right)),
                TokenType::BANG_EQUAL => Ok(Object::Boolean(left != right)),
                TokenType::GREATER_EQUAL => Ok(Object::Boolean(left >= right)),
                TokenType::GREATER => Ok(Object::Boolean(left > right)),
                _ => Err(RuntimeError::new(
                    "Invalid binary operator for numbers.".to_string(),
                    operator.token_type,
                )),
            },
            (Object::String(left), Object::String(right)) => match operator.token_type {
                TokenType::PLUS => Ok(Object::String(left + right.as_str())),
                TokenType::EQUAL_EQUAL => Ok(Object::Boolean(left == right)),
                TokenType::BANG_EQUAL => Ok(Object::Boolean(left != right)),
                _ => Err(RuntimeError::new(
                    "Invalid binary operator for strings.".to_string(),
                    operator.token_type,
                )),
            },
            (_, _) if matches!(operator.token_type, TokenType::EQUAL_EQUAL) => {
                Ok(Object::Boolean(false))
            }
            _ => Err(RuntimeError::new(
                "Invalid operands for binary operator.".to_string(),
                operator.token_type,
            )),
        }
    }

    fn visit_grouping(&self, expr: Box<Expr>) -> Result<Object, RuntimeError> {
        self.ensure_literal(expr)
    }

    fn visit_print_stmt(&self, expr: Box<Expr>) -> Result<Expr, RuntimeError> {
        match *expr {
            Expr::Literal { value } => Ok(Expr::Literal { value }),
            Expr::Unary { operator, right } => {
                let value = self.visit_unary(operator, right)?;
                Ok(Expr::Literal { value })
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                let value = self.visit_binary(operator, left, right)?;
                Ok(Expr::Literal { value })
            }
            Expr::Grouping { expression } => {
                let value = self.visit_grouping(expression)?;
                Ok(Expr::Literal { value })
            }
            Expr::Variable { value } => {
                let var_res = self.environment.borrow().get_var(value)?.clone();
                Ok(Expr::Literal { value: var_res })
            }
        }
    }

    fn visit_stmt(&self, stmt: Statement) -> Result<Expr, RuntimeError> {
        match stmt {
            Statement::PrintStmt(expr) => self.visit_print_stmt(Box::new(expr)),
            Statement::ExprStmt(expr) => unreachable!()
        }
    }
    fn visit_var_decl(&self, decl: Box<Expr>) -> Result<Expr, RuntimeError> {
        match *decl {
            Expr::Unary { operator, right } => match *right {
                Expr::Variable { value: obj } => {
                    self.environment.borrow_mut().set_var(
                        obj.clone(), Object::Nil,
                    );
                    Ok(Expr::Variable { value: obj })
                }
                Expr::Binary { operator, left, right } => {
                    let obj = self.ensure_literal(right)?;
                    if let Expr::Variable { value } = *left {
                        self.environment.borrow_mut().set_var(
                            value, obj.clone(),
                        );
                        return Ok(Expr::Variable { value: obj.to_string() });
                    }
                    unreachable!();
                }
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }
}

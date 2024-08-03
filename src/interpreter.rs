use crate::parser::{Expr, Object};
use crate::token::TokenType;

trait Visitor {
    fn visit_unary(&self, operator: TokenType, right: Box<Expr>) -> Object;
    fn visit_expr(&self, expr: Expr) -> Object;
    fn visit_binary(&self, operator: TokenType, left: Box<Expr>, right: Box<Expr>) -> Object;
    
    fn visit_grouping(&self, expr: Box<Expr>) -> Object;
}
pub(crate) struct Interpreter;

impl Interpreter {
    pub(crate) fn new() -> Self {
        Interpreter {}
    }
    pub(crate) fn interpret(&self, expr: Expr) -> Expr {
        match expr {
            Expr::Literal { value } => Expr::Literal { value },
            Expr::Unary { operator, right } => Expr::Literal {
                value: self.visit_unary(operator.token_type, right),
            },
            Expr::Binary {
                operator,
                left,
                right,
            } => Expr::Literal {
                value: self.visit_binary(operator.token_type, left, right),
            },
            Expr::Grouping {
                expression
            } => Expr::Literal {
                value: self.visit_grouping(expression)
            }
        }
    }

    fn ensure_literal<'a, 'b>(&'b self, mut expr: Box<Expr<'a>>) -> Object
    where
        'b: 'a,
    {
        while !matches!(*expr, Expr::Literal { .. }) {
            expr = Box::new(self.interpret(*expr));
        }

        if let Expr::Literal { value } = *expr {
            value
        } else {
            unreachable!() // We ensured it's a Literal in the loop
        }
    }
}

impl Visitor for Interpreter {
    fn visit_unary(&self, operator: TokenType, right: Box<Expr>) -> Object {
        let right_value = self.ensure_literal(right);
        match operator {
            TokenType::BANG => match right_value {
                Object::Boolean(b) => Object::Boolean(!b),
                _ => Object::Nil,
            },
            TokenType::MINUS => match right_value {
                Object::Number(n) => Object::Number(-n),
                _ => Object::Nil,
            },
            _ => Object::Nil,
        }
    }
    fn visit_expr(&self, expr: Expr) -> Object {
        if let Expr::Literal { value } = expr {
            value
        } else {
            Object::Nil
        }
    }

    fn visit_binary(&self, operator: TokenType, left: Box<Expr>, right: Box<Expr>) -> Object {
        let left_value = self.ensure_literal(left);
        let right_value = self.ensure_literal(right);

        match (left_value, right_value) {
            (Object::Number(left), Object::Number(right)) => match operator {
                TokenType::PLUS => Object::Number(left + right),
                TokenType::MINUS => Object::Number(left - right),
                TokenType::STAR => Object::Number(left * right),
                TokenType::SLASH => Object::Number(left / right),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    fn visit_grouping(&self, expr: Box<Expr>) -> Object {
        self.ensure_literal(expr)
    }
}

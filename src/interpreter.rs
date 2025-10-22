use crate::expr::*;
use crate::object::*;
use crate::error::*;
use crate::token_type::*;

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        match &expr.value {
            Some(val) => Ok(val.clone()),
            None => Ok(Object::Nil),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        expr.expression.accept(self)
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = expr.right.accept(self)?;

        match expr.operator.ttype {
            TokenType::Minus => {
                if let Object::Num(n) = right {
                    Ok(Object::Num(-n))
                } else {
                    Err(LoxError::RuntimeError("Operand must be a number.".to_string()))
                }
            }
            TokenType::Bang => {
                Ok(Object::False) // Simplified for demonstration
            }
            _ => Err(LoxError::RuntimeError("Unknown unary operator.".to_string())),
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = expr.left.accept(self)?;
        let right = expr.right.accept(self)?;

        match expr.operator.ttype {
            TokenType::Plus => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Num(l + r))
                } else {
                    Err(LoxError::RuntimeError("Operands must be numbers.".to_string()))
                }
            }
            _ => Err(LoxError::RuntimeError("Unknown binary operator.".to_string())),
        }
    }
}   
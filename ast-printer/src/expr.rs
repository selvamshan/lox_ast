use crate::token::{Token, Object};
use crate::error::{LoxError};

pub enum Expr {
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxError> {
        match self {
            Expr::Literal(expr) => expr.accept(visitor),
            Expr::Grouping(expr) => expr.accept(visitor),
            Expr::Unary(expr) => expr.accept(visitor),
            Expr::Binary(expr) => expr.accept(visitor),
        }
    }
}

pub struct LiteralExpr {
    pub value: Option<Object>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<T, LoxError>;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<T, LoxError>;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<T, LoxError>;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<T, LoxError>;
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
    }
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }
}


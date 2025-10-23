use crate::expr::*;
use crate::token::*;
use crate::error::*;
use crate::token_type::*;
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> Result<String, LoxError> {
        expr.accept(self)
    }

    fn paranthesize(&mut self, name: &String, exprs: &[&Box<Expr>]) -> Result<String, LoxError> {
        
        let mut builder = format!("({}", name);
        for expr in exprs {
            builder = format!("{} {}", builder, expr.accept(self)?);
        }
        builder = format!("{})", builder);
        Ok(builder)
    }
    
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<String, LoxError> {
        if let Some(value) = &expr.value {
            Ok(value.to_string())
        } else {
            Ok("nil".to_string())
        }        
             
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<String, LoxError> {
       
        self.paranthesize(&"group".to_string(), &[&expr.expression])
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<String, LoxError> {
       
        self.paranthesize(&expr.operator.lexeme, &[&expr.right])
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<String, LoxError> {
        
        self.paranthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_variable_expr(&mut self, _expr: &VariableExpr) -> Result<String, LoxError> {
        Ok("".to_string())
    }
}

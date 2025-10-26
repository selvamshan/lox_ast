use crate::error::*;
use crate::expr::*;
use crate::token::*;
use crate::object::*;
use crate::token_type::*;
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> Result<String, LoxResult> {
        expr.accept(self)
    }

    fn paranthesize(&mut self, name: &String, exprs: &[&Expr]) -> Result<String, LoxResult> {
        let mut builder = format!("({}", name);
        for expr in exprs {
            builder = format!("{} {}", builder, expr.accept(self)?);
        }
        builder = format!("{})", builder);
        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_call_expr(&mut self, expr: &CallExpr) -> Result<String, LoxResult> {
       Ok("nil".to_string())
    }
    fn visit_logical_expr(&mut self, _expr: &LogicalExpr) -> Result<String, LoxResult> {
         Ok("nil".to_string())
    }
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<String, LoxResult> {
        if let Some(value) = &expr.value {
            Ok(value.to_string())
        } else {
            Ok("nil".to_string())
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<String, LoxResult> {
        self.paranthesize(&"group".to_string(), &[&expr.expression])
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<String, LoxResult> {
        self.paranthesize(&expr.operator.lexeme, &[&expr.right])
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<String, LoxResult> {
        self.paranthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_variable_expr(&mut self, _expr: &VariableExpr) -> Result<String, LoxResult> {
        Ok("".to_string())
    }

    fn visit_assign_expr(&mut self, _expr: &AssignExpr) -> Result<String, LoxResult> {
        Ok("".to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_visit_literal_expr() {
        let mut ast = AstPrinter;
        let expr = &LiteralExpr { value: Some(Object::Num(20.0)) };
        let res = ast.visit_literal_expr(expr).unwrap();
        assert_eq!(res, "20");
    }
}
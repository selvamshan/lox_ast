#![allow(dead_code, unused_imports)]
mod expr;
use expr::*;
mod token;
use token::*;
mod error;
use error::*;
mod token_type;
use token_type::*;


struct AstPrinter;

impl AstPrinter {
    fn print(&mut self, expr: &Expr) -> Result<String, LoxError> {
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
}

fn check_print() {
    let expression = Expr::Binary( BinaryExpr {
        left: Box::new( Expr::Unary( UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new( Expr::Literal( LiteralExpr {
                value: Some( Object::Num(123.0) )
            }) )
        }) ),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new( Expr::Grouping( GroupingExpr {
            expression: Box::new( Expr::Literal( LiteralExpr {
                value: Some( Object::Num(45.67) )
            }) )
        }) )
    });

    let mut printer = AstPrinter;
    match printer.print(&expression) {
        Ok(result) => println!("{}", result),
        Err(e) => e.report("".to_string()),
    }
}

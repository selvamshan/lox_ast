use crate::expr::*;
use crate::object::*;
use crate::error::*;
use crate::token::Token;
use crate::token_type::*;

pub struct Interpreter {}

impl Interpreter {
   fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxError> {
       expr.accept(self)
   }
   fn is_truthy(&self, obj: &Object) -> bool {
       match obj {
           Object::Nil => false,
           Object::Bool(false) => false,
           _ => true,
       }
   }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        match &expr.value {
            Some(val) => Ok(val.clone()),
            None => Ok(Object::Nil),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        //expr.expression.accept(self)
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => {
                if let Object::Num(n) = right {
                    Ok(Object::Num(-n))
                } else {
                    Ok(Object::Nil)
                }
            },
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),
        
            _ => Err(LoxError::error(expr.operator.line, "Unreachable code.")),
        }
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;//expr.left.accept(self)?;
        let right = self.evaluate(&expr.right)?;
        let op = expr.operator.token_type();
        
        let result = match(left, right) {
            (Object::Num(l), Object::Num(r)) => {
                match op {
                    TokenType::Plus => Object::Num(l + r),  
                    TokenType::Minus => Object::Num(l - r),
                    TokenType::Star => Object::Num(l * r),    
                    TokenType::Slash => Object::Num(l / r),
                    TokenType::Greater => Object::Bool(l > r),
                    TokenType::GreaterEqual => Object::Bool(l >= r),
                    TokenType::Less =>  Object::Bool(l < r),
                    TokenType::LessEqual => Object::Bool(l <= r),
                    TokenType::EqualEqual => Object::Bool(l == r),
                    TokenType::BangEqual => Object::Bool(l != r),
                    _ => return Err(LoxError::error(expr.operator.line, "Unknown binary operator." )),
                }
            },
            (Object::Str(l), Object::Str(r)) => {
                match op {
                    TokenType::Plus => Object::Str(format!("{}{}", l, r)),
                    TokenType::EqualEqual => Object::Bool(l == r),
                    TokenType::BangEqual => Object::Bool(l != r),
                    _ => Object::ArithmeticError
                }
            },
            (Object::Bool(l), Object::Bool(r)) => {
                match op {
                    TokenType::EqualEqual => Object::Bool(l == r),
                    TokenType::BangEqual => Object::Bool(l != r),
                    _ => Object::ArithmeticError
                }
            },
            (Object::Nil, Object::Nil) => {
                match op {
                    TokenType::EqualEqual => Object::Bool(true),
                    TokenType::BangEqual => Object::Bool(false),
                    _ => Object::ArithmeticError
                }
            },
            (Object::Nil, _) => {
                match op {
                    TokenType::EqualEqual => Object::Bool(false),
                    TokenType::BangEqual => Object::Bool(true),
                    _ => Object::ArithmeticError
                }
            },
            _ => return Err(LoxError::runtime_error(&expr.operator, "Operands must be two numbers or two strings.")),
        };
        
        if result == Object::ArithmeticError {
            Err(LoxError::runtime_error(&expr.operator, "Operands must be numbers."))
        } else {
            Ok(result)
        }
        
        
    }

   
}   

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;
    use crate::token::*;
    
    fn make_literal_str(value:&str) -> Box<Expr> {
       Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Str(value.to_string())) }))       
    }

    fn make_literal_num(value:f64) -> Box<Expr> {
       Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Num(value)) }))       
    }

    #[test]
    fn test_interpreter_literal() {
        let mut interpreter = Interpreter {};
        let expr = LiteralExpr { value: Some(Object::Num(42.0)) } ;
        let result = interpreter.visit_literal_expr(&expr).unwrap();
        assert_eq!(result, Object::Num(42.0));
    }
    #[test]
    fn test_interpreter_grouping() {
        let mut interpreter = Interpreter {};
        let expr = GroupingExpr { expression:  make_literal_num(3.14) } ;
        let result = interpreter.visit_grouping_expr(&expr).unwrap();
        assert_eq!(result, Object::Num(3.14));
    }

    #[test]
    fn test_uninary_minus() {
        let mut interpreter = Interpreter {};
        let expr =  UnaryExpr { operator: Token { ttype: TokenType::Minus, lexeme: "-".to_string(), line: 1, literal: None }, right: make_literal_num(5.0) } ;
        let result = interpreter.visit_unary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(-5.0)));
    }

    #[test]
    fn test_uniary_not() {
        let mut interpreter = Interpreter {};
        let expr =  UnaryExpr { operator: Token { ttype: TokenType::Bang, lexeme: "!".to_string(), line: 1, literal: None }, right: Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Bool(false)) })) } ;
        let result = interpreter.visit_unary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_binary_plus() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Num(10.0)) })), 
            operator: Token { ttype: TokenType::Plus, lexeme: "+".to_string(), line: 1, literal: None }, 
            right: Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Num(15.0)) })) 
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(25.0)));
    }

    #[test]
    fn test_binary_plus_type_error() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Num(10.0)) })), 
            operator: Token { ttype: TokenType::Plus, lexeme: "+".to_string(), line: 1, literal: None }, 
            right: Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Bool(true)) })) 
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_binary_minus() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Num(20.0)) })), 
            operator: Token { ttype: TokenType::Minus, lexeme: "-".to_string(), line: 1, literal: None }, 
            right: Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Num(5.0)) })) 
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(15.0)));
    }

    #[test]
    fn test_binary_minus_type_error() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Num(10.0)) })), 
            operator: Token { ttype: TokenType::Minus, lexeme: "-".to_string(), line: 1, literal: None }, 
            right: Box::new(Expr::Literal( LiteralExpr { value: Some(Object::Bool(true)) })) 
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_binary_star() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_num(4.0), 
            operator: Token { ttype: TokenType::Star, lexeme: "*".to_string(), line: 1, literal: None }, 
            right: make_literal_num(2.5)
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(10.0)));
    }

    #[test]
    fn test_binary_slash() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_num(10.0), 
            operator: Token { ttype: TokenType::Slash, lexeme: "/".to_string(), line: 1, literal: None }, 
            right: make_literal_num(2.0)
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(5.0)));
    }

    #[test]
    fn test_binary_wrong_operator() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_num(10.0), 
            operator: Token { ttype: TokenType::Bang, lexeme: "!".to_string(), line: 1, literal: None }, 
            right: make_literal_num(2.0)
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_strign_concat() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_str("hello, "), 
            operator: Token { ttype: TokenType::Plus, lexeme: "+".to_string(), line: 1, literal: None }, 
            right: make_literal_str("world")
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Str("hello, world".to_string())));
    }

    #[test]
    fn test_binary_grater() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_num(10.0), 
            operator: Token { ttype: TokenType::Greater, lexeme: ">".to_string(), line: 1, literal: None }, 
            right: make_literal_num(2.0)
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_binary_less() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_num(1.0), 
            operator: Token { ttype: TokenType::Less, lexeme: "<".to_string(), line: 1, literal: None }, 
            right: make_literal_num(2.0)
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_binary_greater_equal() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_num(2.0), 
            operator: Token { ttype: TokenType::GreaterEqual, lexeme: ">=".to_string(), line: 1, literal: None }, 
            right: make_literal_num(2.0)
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_binary_less_equal() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_num(1.0), 
            operator: Token { ttype: TokenType::LessEqual, lexeme: "<=".to_string(), line: 1, literal: None }, 
            right: make_literal_num(2.0)
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_binary_equal_equal() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_num(2.0), 
            operator: Token { ttype: TokenType::EqualEqual, lexeme: "==".to_string(), line: 1, literal: None }, 
            right: make_literal_num(2.0)
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_binary_equal_equal_str() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_str("hello"), 
            operator: Token { ttype: TokenType::EqualEqual, lexeme: "==".to_string(), line: 1, literal: None }, 
            right: make_literal_str("hello")
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test] 
    fn test_binary_bang_equal() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_num(2.0), 
            operator: Token { ttype: TokenType::BangEqual, lexeme: "!=".to_string(), line: 1, literal: None }, 
            right: make_literal_num(3.0)
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_binary_bang_equal_str() {
        let mut interpreter = Interpreter {};
        let expr =  BinaryExpr { 
            left: make_literal_str("hello"), 
            operator: Token { ttype: TokenType::BangEqual, lexeme: "!=".to_string(), line: 1, literal: None }, 
            right: make_literal_str("world")
        } ;
        let result = interpreter.visit_binary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_is_truthy() {
        let interpreter = Interpreter {};
        assert_eq!(interpreter.is_truthy(&Object::Nil), false);
        assert_eq!(interpreter.is_truthy(&Object::Bool(false)), false);
        assert_eq!(interpreter.is_truthy(&Object::Bool(true)), true);
        assert_eq!(interpreter.is_truthy(&Object::Num(0.0)), true);
        assert_eq!(interpreter.is_truthy(&Object::Str("hello".to_string())), true);
    }
}
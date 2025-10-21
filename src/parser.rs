use crate::token_type::*;
use crate::token::*;
use crate::error::*;
use crate::expr::*;
         
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }

    fn peek(&self) -> &Token {
       &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

 
    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().ttype == ttype
    } 

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }  

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary( BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }        

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary( BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary( BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary( BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary( UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }


    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expr::Literal( LiteralExpr { value: Some(Object::False) }));
        }
        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::Literal( LiteralExpr { value: Some(Object::True) }));
        }
        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Expr::Literal( LiteralExpr { value: Some(Object::Nil) }));
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]){
            return Ok(Expr::Literal( LiteralExpr { value: self.previous().literal.clone() }) );
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping( GroupingExpr { expression: Box::new(expr) }));
        }

        // if self.match_tokens(&[TokenType::Number]) {
        //     let value = match &self.previous().literal {
        //         Some(Object::Num(n)) => *n,
        //         _ => return Err(LoxError::error(self.previous().line, "Expected number literal.".to_string())),
        //     };
        //     return Ok(Expr::Literal( LiteralExpr { value: Some(Object::Num(value)) }));
        // }
        // if self.match_tokens(&[TokenType::String]) {
        //     let value = match &self.previous().literal {
        //         Some(Object::Str(s)) => s.clone(),
        //         _ => return Err(LoxError::error(self.previous().line, "Expected string literal.".to_string())),
        //     };
        //     return Ok(Expr::Literal( LiteralExpr { value: Some(Object::Str(value)) }));
        // }
        // if self.match_tokens(&[TokenType::LeftParen]) {
        //     let expr = self.expression()?;
        //     if !self.match_tokens(&[TokenType::RightParen]) {
        //         return Err(LoxError::error(self.peek().line, "Expected ')' after expression.".to_string()));
        //     }
        //     return Ok(Expr::Grouping( GroupingExpr { expression: Box::new(expr) }));
        // }

        Err(LoxError::error(self.peek().line, "Expected expression.".to_string()))
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<&Token, LoxError> {
        if self.check(&ttype) {
            return Ok(self.advance());
        }

        Err(LoxError::error(self.peek().line, message.to_string()))
    }

        
    

    
}
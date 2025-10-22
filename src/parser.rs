use crate::token_type::*;
use crate::object::*;
use crate::token::*;
use crate::error::*;
use crate::expr::*;
use crate::stmt::*;
         
pub struct Parser<'a> {
     pub
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?)
        }
        // match self.expression() {
        //     Ok(expr) => Some(expr),
        //     Err(_) => None
        // }
        Ok(statements)
    }

  

    fn is_at_end(&self) -> bool {
        self.peek().is(&TokenType::Eof)
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
        &self.peek().token_type() == ttype
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

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_tokens(&[TokenType::Print]) {
            return self.print_statement();
        }
        return self.expresion_statement();

    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value")?;
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    fn expresion_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().dup();
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
            return Ok(Expr::Literal( LiteralExpr { value: Some(Object::Bool(false)) }));
        }
        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::Literal( LiteralExpr { value: Some(Object::Bool(true)) }));
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

        
        Err(LoxError::error(self.peek().line, "Expected expression."))
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<&Token, LoxError> {
        if self.check(&ttype) {
            return Ok(self.advance());
        }

        //Err(LoxError::error(self.peek().line, message.to_string()))
        Err(Parser::error(self.peek(), message))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().is(&TokenType::Semicolon) {
                return;
            }

            match self.peek().token_type() {
                TokenType::Class |
                TokenType::Fun |
                TokenType::Var |
                TokenType::For |
                TokenType::If |
                TokenType::While |
                TokenType::Print |
                TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

        
    fn error(token: &Token, message: &str) -> LoxError {
        LoxError::parse_error(token, message)
    }

    
}
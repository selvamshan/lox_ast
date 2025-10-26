use crate::object::Object;
use crate::{error::LoxError, token::*, token_type::TokenType};
use std::f32::consts;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut has_error: Option<LoxError> = None;

        while !self.is_at_end() {
            self.start = self.current;
            // Here would be the logic to scan a single token
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report("");
                    has_error = Some(e);
                }
            }
        }
        self.tokens.push(Token::eof(self.line));
        if let Some(e) = has_error {
            return Err(e);
        }
        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn is_match(&mut self, expected: char) -> bool {
        if let Some(c) = self.source.chars().nth(self.current) {
            if c != expected {
                return false;
            }
            self.current += 1;
            return true;
        }
        false
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn scan_string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // Handle unterminated string error
            return Err(LoxError::error(self.line, "Unterminated string."));
        }

        // The closing ".
        self.advance();

        // TODO: Handle escape sequences here

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(Object::Str(value.to_string())));
        Ok(())
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
        //('0'..='9').contains(&c)//c >= '0' && c <= '9'
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current];
        let number_value = value.parse::<f64>().unwrap();
        self.add_token(TokenType::Number, Some(Object::Num(number_value)));      
    }

    fn is_alpha(&self, c: char) -> bool {       
        c.is_ascii_uppercase() || c.is_ascii_lowercase() 
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn keyword(check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        if let Some(ttype) = Scanner::keyword(text) {
            self.add_token(ttype.clone(), None);
        } else {
            self.add_token(TokenType::Identifier, None);
        }
    }
    fn scan_comment(&mut self) -> Result<(), LoxError> {
        loop {
            match self.peek() {
                '*' => {
                    if self.peek_next() == '/' {
                        self.advance(); // consume '*'
                        self.advance(); // consume '/'
                        return Ok(());
                    } else {
                        self.advance();
                    }
                }
                '/' => {
                    if self.peek_next() == '*' {
                        self.advance(); // consume '/'
                        self.advance(); // consume '*'
                        // Nested comment
                        self.scan_comment()?;
                    } else {
                        self.advance();
                    }
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '\0' => {
                    return Err(LoxError::error(self.line, "Unterminated comment"));
                } // End of source
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        // Placeholder for scanning a single token
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::SemiColon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => match self.is_match('=') {
                true => self.add_token(TokenType::BangEqual, None),              
                false => self.add_token(TokenType::Bang, None),               
            },
            '=' => match self.is_match('=') {
                true => self.add_token(TokenType::Equal, None),               
                false =>  self.add_token(TokenType::Assign, None),                  
            },
            '>' => match self.is_match('=') {
                true => self.add_token(TokenType::GreaterEqual, None),                
                false => self.add_token(TokenType::Greater, None),  
            },
            '<' => match self.is_match('=') {
                true => self.add_token(TokenType::LessEqual, None),
                false => self.add_token(TokenType::Less, None),            
            },
            '/' => {
                if self.is_match('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.is_match('*') {
                    // Multiline comment
                    self.scan_comment()?;
                } else {
                    self.add_token(TokenType::Slash, None);
                }
                
            }

            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;                
            }
            '"' => {
                // String literal scanning would go here
                self.scan_string()?;                 
            }
            '0'..='9' => {             
                self.number();
            }
            _ if self.is_alpha(c) || c == '_' => {
                self.identifier();
            }
            _ => {
                LoxError::error(self.line, "Unexpected character.");
            }
        };
        Ok(())
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Object>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(ttype, text.to_string(), literal, self.line));
    }
}

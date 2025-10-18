use crate::{error::Loxerror, token::*, token_type::TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}   

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, Loxerror> {
        // Placeholder implementation
        while !self.is_at_end() {
            self.start = self.current;
            // Here would be the logic to scan a single token
            match self.scan_token() {
                Ok(_) => {},
                Err(e) => e.report("".to_string()),
            }
         
        }
        self.tokens.push(Token::eof(self.line));
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

    fn next_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // Handle unterminated string error
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(Object::Str(value.to_string())));
    }

    fn scan_token(&mut self) -> Result<(), Loxerror> {
        // Placeholder for scanning a single token
        let c = self.advance();
        match c {
            '(' => Ok(self.add_token(TokenType::LeftParen, None)),
            ')' => Ok(self.add_token(TokenType::RightParen, None)),
            '{' => Ok(self.add_token(TokenType::LeftBrace, None)),
            '}' => Ok(self.add_token(TokenType::RightBrace, None)),
            ',' => Ok(self.add_token(TokenType::Comma, None)),
            '.' => Ok(self.add_token(TokenType::Dot, None)),
            '-' => Ok(self.add_token(TokenType::Minus, None)),
            '+' => Ok(self.add_token(TokenType::Plus, None)),
            ';' => Ok(self.add_token(TokenType::Semicolon, None)),
            '*' => Ok(self.add_token(TokenType::Star, None)),
            '!'  => match self.next_char('=') {
                true => Ok(self.add_token(TokenType::BangEqual, None)),
                false => Ok(self.add_token(TokenType::Bang, None)),
            },
            '=' => match self.next_char('=') {
                true => Ok(self.add_token(TokenType::EqualEqual, None)),
                false => Ok(self.add_token(TokenType::Equal, None)),
            },
            '>' => match self.next_char('=') {
                true => Ok(self.add_token(TokenType::GreaterEqual, None)),
                false => Ok(self.add_token(TokenType::Greater, None)),
            },
            '<' => match self.next_char('=') {
                true => Ok(self.add_token(TokenType::LessEqual, None)),
                false => Ok(self.add_token(TokenType::Less, None)),
            },
            '/' => {
                if self.next_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                        
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
                Ok(())
            }

            ' '| '\r' | '\t' => {
                // Ignore whitespace.
                Ok(())
            }
            '\n' => {
                self.line += 1;
                Ok(())
            },
            '"' => {
                // String literal scanning would go here
                self.scan_string();
                Ok(())
            },
          
            _ => {
                return Err(Loxerror::error(self.line, "Unexpected character.".to_string()));
            }
        }
         
    }

 

    fn add_token(&mut self, ttype: TokenType, literal: Option<Object>)  {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(ttype, text.to_string(), literal, self.line));
    }
}
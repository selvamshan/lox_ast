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
            self.scan_token();
            self.current += 1; // Dummy increment to avoid infinite loop
            
        }
        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        // Placeholder for scanning a single token
        let c = self.source.chars().nth(self.current).unwrap();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            // Add more cases for other tokens
            _ => {}
        }
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Object>) {
        let text = &self.source[self.start..self.current + 1];
        self.tokens.push(Token::new(ttype, text.to_string(), literal, self.line));
    }
}
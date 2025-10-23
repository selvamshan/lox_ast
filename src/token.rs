use core::fmt;

use crate::object::Object;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Self {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    pub fn is(&self, ttype: &TokenType) -> bool {
        &self.ttype == ttype
    }

    pub fn token_type(&self) -> TokenType {
        self.ttype.clone()
    }

    pub fn as_string(&self) -> String {
        self.lexeme.clone()
    }

    pub fn dup(&self) -> Self {
        Self {
            ttype: self.ttype.clone(),
            lexeme: self.lexeme.clone(),
            literal: self.literal.clone(),
            line: self.line,
        }
    }

    pub fn eof(line: usize) -> Self {
        Self {
            ttype: TokenType::Eof,
            lexeme: "EOF".to_string(),
            literal: None,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.lexeme,
            match &self.literal {
                Some(lit) => format!("{}", lit),
                None => "none".to_string(),
            }
        )
    }
}

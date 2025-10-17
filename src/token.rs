use core::fmt;

use crate::token_type::TokenType;


#[derive(Debug, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Num(n) => write!(f, "{}", n),
            Object::Str(s) => write!(f, "{}", s),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize   
}

impl  Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line:usize) -> Self {
        Self { ttype, lexeme, literal, line }
    }

    pub fn eof(line:usize) -> Self {
        Self {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.ttype, self.lexeme, match &self.literal {
            Some(lit) => format!("{}", lit),
            None => "none".to_string()
        }
    )
    }
}
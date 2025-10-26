use crate::token::{self, Token};
use crate::token_type::TokenType;

#[derive(Debug)]
pub enum LoxResult {
    ParseError {token:Token, message: String},
    RuntimeError {token:Token, message: String},
    Error {line:usize, message: String},
    SystemError {message:String},
    Break
}

// #[derive(Debug)]
// pub struct LoxError {
//     token: Option<Token>,
//     line: usize,
//     message: String,
// }

impl LoxResult {
    pub fn error(line: usize, message: &str) -> LoxResult {
        let err = LoxResult::Error {
             line, 
             message: message.to_string() 
            } ;
        
        err.report("");
        err
    }

    pub fn parse_error(token: &Token, message: &str) -> LoxResult {
        let err = LoxResult::ParseError { 
            token: token.dup(),
            message: message.to_string()
        }; 
        err.report("");
        err
    }

    pub fn runtime_error(token: &Token, message: &str) -> LoxResult {
        let err = LoxResult::RuntimeError {           
            token: token.dup(),          
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn system_error(message:&str) -> LoxResult {
        let err = LoxResult::SystemError { message:  message.to_string()};
        err.report("");
        err
    }

    pub fn report(&self, loc: &str) {
        match self {
            LoxResult::ParseError { token, message } => {
                if token.is(&TokenType::Eof) {
                    eprintln!("[line {}] Error at end: {}", token.line, message);
                } else {
                    eprintln!(
                        "[line {}] Error at '{}': {}",
                        token.line,
                        token.as_string(),
                        message
                    );
                }
            }
            LoxResult::RuntimeError { token, message } => {
                if token.is(&TokenType::Eof) {
                    eprintln!("[line {}] Error at end: {}", token.line, message);
                } else {
                    eprintln!("Line {} at '{}' {}", token.line, token.as_string(), message,);
                }
            }
            LoxResult::Error { line, message } => {
                eprintln!("[line {}] Error{}: {}", line, loc, message);
            },
            LoxResult::SystemError { message } => {
                eprintln!("System Error: {message}");
            },
            LoxResult::Break => {}

        };
    }
}

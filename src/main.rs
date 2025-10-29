//https://github.com/UncleScientist/lox-ast
#![allow(dead_code, unused_imports)]
mod error;
use error::*;
mod token_type;
use token_type::*;
mod object;
mod token;
use token::*;
mod scanner;
use scanner::*;
mod parser;
use parser::*;
mod expr;
mod interpreter;
mod stmt;
use interpreter::*;
//mod ast_printer;
mod environment;
mod callable;
mod native_functions;
mod lox_function;
//mod resolver;
//use ast_printer::AstPrinter;

use std::env::args;
use std::rc::Rc;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write, stdout};

fn main() {
    let args: Vec<String> = args().collect();
    let mut lux = Lux::new();
    println!("args: {:?}, {}", args, args.len());
    if args.len() > 2 {
        println!("Usage: lox ast [Script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        let _ = lux.run_file(&args[1]);
    } else {
        lux.run_prompt()
    }
}

struct Lux {
    interpreter: Interpreter,
}

impl Lux {
    pub fn new() -> Self {
        Lux {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, path: &String) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf.as_str()).is_err() {
            std::process::exit(65);
        }
        // match self.run(buf.as_str()) {
        //     Ok(_) => (),
        //     Err(_e) => {
        //     //e.report("".to_string());
        //     std::process::exit(65);
        //     }

        Ok(())
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        print!(">  ");
        stdout().flush().unwrap();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                match self.run(line.as_str()) {
                    Ok(_) => (),
                    Err(_) => {
                        // ingnore: error was already reported
                    }
                }
            } else {
                break;
            }
            print!(">  ");
            stdout().flush().unwrap();
        }
    }

    pub fn run(&mut self, source: &str) -> Result<(), LoxResult> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let statements = Rc::new(parser.parse()?);
        if parser.success() {
             self.interpreter.interpret(&statements);
            
        } 
        Ok(())
        
    }
}

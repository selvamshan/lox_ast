#![allow(dead_code, unused_imports)]
mod error;
use error::*;
mod token_type;
use token_type::*;
mod token;
use token::*;
mod scanner;
use scanner::*;

use std::env::args;
use std::io::{self, BufReader, BufRead, Read, Write, stdout};
use std::fs::File;




fn main() {
    let args:Vec<String> = args().collect();
    println!("args: {:?}, {}", args, args.len());
    if args.len() > 2 {
        println!("Usage: lox ast [Script]");
       std::process::exit(64);
    } 
    else if args.len() == 2{
        let _ = run_file(&args[1]);
    } else {
       run_prompt()
    }
  
}

fn run_file(path:&String) -> io::Result<()> {
    
    let buf = std::fs::read_to_string(path)?;
    match run(buf.as_str()) {
        Ok(_) => (),
        Err(e) => {
           e.report("".to_string());
           std::process::exit(65);
        }
    } 
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("* ");
    stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
          match run(line.as_str()){
            Ok(_) => (),
            Err(_) => {
                // ingnore: error was already reported 
            }
          }
        } else {
            break;
        }
         print!("* ");
        stdout().flush().unwrap();
    }
}



fn run(source: &str) -> Result<(), Loxerror>{
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}





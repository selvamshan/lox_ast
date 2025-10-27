
use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;

use crate::environment::*;
use crate::object::*;
use crate::token::*;
use crate::callable::*;
use crate::interpreter::*;
use crate::error::*;
use crate::stmt::*;

pub struct LoxFunction {
    name: Token,
    params: Rc<Vec<Token>>,
    body: Rc<Vec<Stmt>>,
    closure: Rc<RefCell<Environment>>
}

impl LoxFunction {
    pub fn new(declaration: &FunctionStmt, closure:&Rc<RefCell<Environment>>) -> Self {

        Self { 
            name: declaration.name.dup(),
            params: Rc::clone(&declaration.params),
            body : Rc::clone(&declaration.body),
            closure: Rc::clone(closure)
         }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter:&mut Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
       
        let mut e = Environment::new_with_enclosing(Rc::clone(&self.closure));

        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            
            e.define(&param.as_string(), arg.clone());
        }
        
        match interpreter.exceute_block(&self.body, e){
            Err(LoxResult::RetrunValue{value}) => Ok(value),
            Err(e) => Err(e),
            Ok(_) => Ok(Object::Nil)

        }
        //Ok(Object::Nil)   
      
    }

    fn arity(&self) -> usize {       
            self.params.len()      
      }
        
    
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {       
            write!(f, "<fn {}>", self.name.as_string())       
    }
}
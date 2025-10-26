use crate::interpreter;
use crate::interpreter::*;
use crate::object::*;
use crate::error::*;

use std::fmt;
use core::fmt::Debug;
use std::rc::Rc;

#[derive(Clone)]
pub struct Callable{
    pub func: Rc<dyn LoxCallable>,
    pub arity: usize,
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
        
    }

}

impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<callable>")
        
    }

}

pub trait LoxCallable {
    fn call(&self, interpreter:&Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
}


   

impl LoxCallable for Callable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        self.func.call(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        self.arity
    }

}


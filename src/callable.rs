use crate::interpreter;
use crate::interpreter::*;
use crate::lox_class::LoxClass;
use crate::object::*;
use crate::error::*;

use std::fmt;
use core::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone)]
pub struct Callable{
    pub func: Rc<dyn LoxCallable>,    
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        //Rc::ptr_eq(&self.func, &other.func)
        std::ptr::eq( 
            Rc::as_ptr(&self.func) as *const(),
            Rc::as_ptr(&other.func)  as *const()
        )
        
    }

}

impl Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<callable>")
    }
}

impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<callable>")
        
    }

}

pub trait LoxCallable {
    fn call(
        &self, 
        
        interpreter:& Interpreter, 
        arguments: Vec<Object>,
        klass: Option<Rc<LoxClass>>
    ) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
}


   




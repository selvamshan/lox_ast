use std::fmt::{Display, Debug, write,self};
use std::rc::Rc;

use crate::callable::*;
use crate::object::*;
use crate::error::*;
use crate::interpreter::*;
use crate::lox_instance::*;


#[derive(Clone,Debug, PartialEq)]
pub struct LoxClass {
   pub name: String
}

impl LoxClass {
    pub fn new(name:&String) -> Self {
        Self{name:name.clone()}
    }

    pub fn instantiate(
        self, 
        _interpreter:&Interpreter, 
        _arguments: Vec<Object>, 
        klass: Option<Rc<LoxClass>>) -> Result<Object, LoxResult> {
            Ok(Object::Instance(Rc::new(LoxInstance::new(klass.unwrap()))))
    }
}



impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn call(&self,
         _interpreter:&Interpreter, 
         _arguments: Vec<crate::object::Object>,
         klass: Option<Rc<LoxClass>>
        ) -> Result<Object, LoxResult> {
        Ok(Object::Instance(Rc::new(LoxInstance::new(klass.unwrap()))))
    }
    fn arity(&self) -> usize {
        0
    }
}
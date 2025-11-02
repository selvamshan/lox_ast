use std::collections::HashMap;
use std::fmt::{Display, Debug, write,self};
use std::fs::metadata;
use std::rc::Rc;

use crate::callable::*;
use crate::lox_function::*;
use crate::object::*;
use crate::error::*;
use crate::interpreter::*;
use crate::lox_instance::*;


#[derive(Clone,Debug, PartialEq)]
pub struct LoxClass {
   pub name: String,
   methods: HashMap<String, Object>
}

impl LoxClass {
    pub fn new(name:&str, methods: HashMap<String, Object>) -> Self {
        Self{
            name:name.to_string(),
            methods,
        }
    }

    pub fn instantiate(
        self, 
        _interpreter:&Interpreter, 
        _arguments: Vec<Object>, 
        klass: Option<Rc<LoxClass>>) -> Result<Object, LoxResult> {
            Ok(Object::Instance(Rc::new(LoxInstance::new(klass.unwrap()))))
    }

    pub fn find_method(&self, name:&str) -> Option<Object> {
        self.methods.get(name).cloned()
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
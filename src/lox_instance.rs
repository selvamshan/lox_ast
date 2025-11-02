use std::collections::hash_map::Entry;
use std::{collections::HashMap, fmt::Display, rc::Rc};
use std::cell::RefCell;

use crate::error::LoxResult;
use crate::lox_class::*;
use crate::object::*;
use crate::error::*;
use crate::token::*;

#[derive(Debug,Clone, PartialEq)]
pub struct LoxInstance {
    klass: Rc<LoxClass>,
    fields: RefCell<HashMap<String , Object>>
}


impl LoxInstance {
    pub fn new(klass: Rc<LoxClass>) -> Self {
        Self { 
            klass : Rc::clone(&klass),
            fields: RefCell::new(HashMap::new())
        }
    }

    pub fn get(&self, name:&Token) -> Result<Object, LoxResult> {
        if let Entry::Occupied(o) = self.fields.borrow_mut().entry(name.as_string()) {
            Ok(o.get().clone())
        } else{
            Err(LoxResult::runtime_error(
                name, 
                &format!("Undefined property '{}'.", name.as_string())))
        }
    }
    
    pub fn set(&self, name:&Token, object: Object)   {
        self.fields.borrow_mut().insert(name.as_string(), object);
    }
}



impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.klass)
    }
}
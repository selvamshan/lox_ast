use core::fmt;
use std::rc::Rc;
use std::time::SystemTime;

use crate::callable::*;
use crate::interpreter::*;
use crate::lox_class::LoxClass;
use crate::object::*;
use crate::error::*;

#[derive(Clone)]
pub struct LoxNative {
    pub func: Rc<dyn LoxCallable>,
}

impl PartialEq for LoxNative {
    fn eq(&self, other: &Self) -> bool {
        //Rc::ptr_eq(&self.func, &other.func)
        std::ptr::eq( 
            Rc::as_ptr(&self.func) as *const(),
            Rc::as_ptr(&other.func)  as *const()
        )
        
    }
}

impl fmt::Debug  for LoxNative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}

impl fmt::Display for LoxNative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}


pub struct Nativeclock;
impl LoxCallable for Nativeclock {
    fn call(
        &self, 
        _interpreter: &Interpreter, 
        _arguments: Vec<Object>,
        _klass: Option<Rc<LoxClass>>
    ) -> Result<Object, LoxResult> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
            Ok(n) => Ok(Object::Num(n.as_millis() as f64)),
            Err(e) => Err(LoxResult::system_error(&format!(
                "Clock return Invalid duration {:?}", e.duration())))
        }
        //Ok(Object::Num(123.456))
    }
    fn arity(&self) -> usize {
        0
    }
}
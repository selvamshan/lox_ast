use std::time::SystemTime;

use crate::callable::*;
use crate::interpreter::*;
use crate::object::*;
use crate::error::*;

pub struct Nativeclock;
impl LoxCallable for Nativeclock {
    fn call(&self, _interpreter:&mut Interpreter, _arguments: Vec<Object>) -> Result<Object, LoxResult> {
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
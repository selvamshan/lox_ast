use core::fmt;


#[derive(Debug, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Num(n) => write!(f, "{}", n),
            Object::Str(s) => write!(f, "{}", s),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}
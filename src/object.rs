use core::fmt;
use std::backtrace;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
    ArithmeticError,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Num(n) => write!(f, "{}", n),
            Object::Str(s) => write!(f, "{}", s),
            Object::Nil => write!(f, "nil"),
            Object::Bool(b) => write!(f, "{}", b),
            Object::ArithmeticError => panic!("Should not print ArithmeticError"),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Bool(b) => *b,
            Object::Nil => false,
            _ => true,
        }
    }

    // pub fn to_string(&self) -> String {
    //     match self {
    //         Object::Num(n) => n.to_string(),
    //         Object::Str(s) => s.clone(),
    //         Object::Nil => "nil".to_string(),
    //         Object::Bool(b) => b.to_string(),
    //         Object::ArithmeticError => "ArithmeticError".to_string(),
    //     }
    // }
}

impl Sub for Object {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(l), Object::Num(r)) => Object::Num(l - r),
            _ => Object::ArithmeticError,
        }
    }
}

impl Add for Object {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(l), Object::Num(r)) => Object::Num(l + r),
            (Object::Str(l), Object::Str(r)) => Object::Str(l + &r),
            _ => Object::ArithmeticError,
        }
    }
}

impl Div for Object {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(l), Object::Num(r)) => Object::Num(l / r),
            _ => Object::ArithmeticError,
        }
    }
}

impl Mul for Object {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(l), Object::Num(r)) => Object::Num(l * r),
            _ => Object::ArithmeticError,
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Object::Num(l), Object::Num(r)) => l.partial_cmp(r),
            (Object::Nil, o) => {
                if o == &Object::Nil {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl PartialEq<f64> for Object {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Object::Num(n) => n == other,
            _ => false,
        }
    }
}

impl PartialEq<String> for Object {
    fn eq(&self, other: &String) -> bool {
        match self {
            Object::Str(s) => s == other,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_object_equality() {
        let obj = Object::Num(5.0);
        assert_eq!(obj, 5.0);
        assert_ne!(obj, 3.0);
    }

    #[test]
    fn test_num_object_comparison() {
        let obj1 = Object::Num(3.0);
        let obj2 = Object::Num(4.0);
        assert!(obj1 < obj2);
        assert!(obj2 > obj1);
        assert_eq!(obj1.partial_cmp(&obj2), Some(std::cmp::Ordering::Less));
    }
}

use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use crate::parser::ast::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    String(String),
    Bool(bool),
}

impl Value {
    pub fn ast_type(&self) -> Type {
        match self {
            Self::Int(_) => Type::Int,
            Self::String(_) => Type::String,
            Self::Bool(_) => Type::Bool,
        }
    }

    pub fn compare(&self, other: &Self) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Bool(x==y)),
            (Value::String(x), Value::String(y)) => Ok(Value::Bool(x==y)),
            (Value::Bool(x), Value::Bool(y)) => Ok(Value::Bool(*x && *y)),
            _ => Err(format!("Cannot compare {} and {}", self, other))
        }
    }
}

impl Add for Value {
    type Output = Result<Value, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x + y)),
            (Value::String(x), Value::String(y)) => Ok(Value::String(x.clone() + y)),
            (Value::Bool(x), Value::Bool(y)) => Ok(Value::Bool(*x && *y)),
            _ => Err(format!("Cannot add {} and {}", self, rhs))
        }
    }
}

impl Sub for Value {
    type Output = Result<Value, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x-y)),
            // (Value::String(x), Value::String(y)) => Ok(Value::String(x+y))
            // (Value::Bool(x), Value::Bool(y)) => Ok(Value::Bool(x+y))
            _ => Err(format!("Cannot subtract {} and {}", self, rhs))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.ast_type(),
            match self {
                Self::String(x) => x.to_string(),
                Self::Int(x) => x.to_string(),
                Self::Bool(x) => x.to_string(),
            }
        )
    }
}

use std::fmt::{Display, Formatter};
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

use std::collections::HashMap;
use crate::parser::ast::FunctionNode;
use super::value::Value;

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variables: HashMap<String, Value>,
    functions: HashMap<String, &'a FunctionNode>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Scope {
            parent: None,
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn from_parent(parent: &'a Scope<'a>) -> Self {
        Scope {
            parent: Some(parent),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, function: &'a FunctionNode) {
        self.functions.insert(function.name.clone(), function);
    }

    pub fn get_function(&self, name: &str) -> Option<&&'a FunctionNode> {
        self.functions.get(name).or_else(||
            self.parent.and_then(|p| p.get_function(name))
        )
    }

    pub fn add_variable(&mut self, name: impl ToString, value: Value) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name).or_else(||
            self.parent.and_then(|p| p.get_variable(name))
        )
    }

    pub fn clone_variable(&self, name: &str) -> Option<Value> {
        self.get_variable(name).map(Clone::clone)
    }
}

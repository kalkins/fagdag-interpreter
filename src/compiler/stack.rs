use crate::compiler::helper::type_size;
use crate::parser::ast::Type;

pub struct StackEntry {
    name: String,
    type_name: Type,
}

pub struct Stack<'a> {
    parent: Option<&'a Stack<'a>>,
    variables: Vec<StackEntry>,
}

impl<'a> Stack<'a> {
    pub fn new() -> Self {
        Stack {
            parent: None,
            variables: vec![],
        }
    }

    pub fn add_variable(&mut self, name: &str, type_name: Type) {
        self.variables.push(
            StackEntry {
                name: name.into(),
                type_name: type_name.clone(),
            }
        )
    }

    pub fn get_variable(&self, name: &str) -> Option<(usize, Type)> {
        let mut offset = 0;

        self.variables.iter().rev().find_map(|var| {
            if var.name == name {
                Some((offset, var.type_name.clone()))
            } else {
                offset += type_size(var.type_name);
                None
            }
        }).or_else(||
            self.parent.and_then(|parent|
                parent.get_variable(name)
            ).map(|(parent_offset, type_name)|
                (offset + parent_offset, type_name)
            )
        )
    }

    pub fn offset(&self, name: &str) -> Option<usize> {
        self.get_variable(&name).map(|x| x.0)
    }

    pub fn size(&self) -> usize {
        self.variables.iter().fold(0, |acc, v| acc + type_size(v.type_name))
    }
}
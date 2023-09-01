use crate::parser::ast::Type;

pub fn type_size(type_name: Type) -> usize {
    match type_name {
        Type::Int => 8,
        Type::Bool => 1,
        Type::String => todo!(),
    }
}

pub fn store_instruction(type_name: Type) -> &'static str {
    match type_size(type_name) {
        1 => "sb",
        4 => "sw",
        8 => "sd",
        x => panic!("Invalid type size {x}")
    }
}

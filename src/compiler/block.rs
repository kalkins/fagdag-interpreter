use crate::compiler::expression::compile_expression;
use crate::compiler::helper::store_instruction;
use crate::compiler::stack::Stack;
use crate::parser::ast::BlockNode;

pub fn compile_block(block: &Vec<BlockNode>, stack: &Stack) -> Result<String, String> {
    block.iter().map(|node| {
        match node {
            BlockNode::VariableDefinition { name, type_name, value } => {
                let offset = stack.offset(&name).ok_or(format!("Could not get offset for variable {}", name))?;

                compile_expression(value, stack).map(|s|
                    s + &format!("\
    {}      t0, -{}(sp)
", store_instruction(type_name.clone()), offset)
                )
            }
            BlockNode::Assignment { .. } => todo!(),
            BlockNode::Expression(_) => todo!(),
            BlockNode::Block(_) => todo!(),
            BlockNode::IfStatement { .. } => todo!(),
            BlockNode::Return(value) => {
                compile_expression(value, stack).map(|s|
                    s + "    mv      a0, t0 # Prepare return variable\n"
                      + "    j       0f     # Jump to end\n"
                )
            },
        }
    }).collect()
}

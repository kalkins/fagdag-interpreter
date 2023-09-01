use crate::compiler::block::compile_block;
use crate::compiler::helper::store_instruction;
use crate::compiler::stack::Stack;
use crate::parser::ast::{BlockNode, FunctionNode, Type};

pub fn compile_function(function: &FunctionNode) -> Result<String, String> {
    let mut stack = Stack::new();

    let return_address_name = "__return_address__";
    stack.add_variable(return_address_name, Type::Int);

    for param in &function.parameters {
        stack.add_variable(&param.name, param.type_name)
    }

    for node in &function.block {
        if let BlockNode::VariableDefinition { name, type_name, .. } = node {
            stack.add_variable(name, *type_name);
        }
    }

    let mut preamble = format!("\
{name}: # Preamble
    addi    sp, sp, -{size}
    sd      ra, -{ra}(sp)
", name=function.name, size=stack.size(), ra=stack.offset(return_address_name).ok_or("Can not find return address offset")?);

    for (i, param) in function.parameters.iter().enumerate() {
        let offset = stack.offset(&param.name).ok_or(format!("Could not get offset for parameter {}", param.name))?;

        preamble.push_str(&format!("\
    {}      a{}, -{}(sp)
", store_instruction(param.type_name), i, offset))
    }

    preamble.push_str("    # Preamble end");

    let postamble = format!("\
0:  # Postamble
    ld      ra, -{ra}(sp)
    addi    sp, sp, {size}
    ret
", size=stack.size(), ra=stack.offset(return_address_name).ok_or("Can not find return address offset")?);

    Ok(format!("{preamble}\n\n{}\n{postamble}", compile_block(&function.block, &stack)?))
}
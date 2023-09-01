use crate::compiler::stack::Stack;
use crate::parser::ast::{ExpressionNode, TermNode};

pub fn compile_expression(expression: &ExpressionNode, stack: &Stack) -> Result<String, String> {
    match expression {
        ExpressionNode::BinaryOperation { .. } => todo!(),
        ExpressionNode::Term(term) => match term {
            TermNode::Integer(x) => {
                Ok(format!("    li      t0, {x} # Load term {x}\n"))
            },
            TermNode::Boolean(_) => todo!(),
            TermNode::Variable(_) => todo!(),
            TermNode::String(_) => todo!(),
        },
    }
}
use crate::parser::ast::{ExpressionNode, TermNode};
use super::scope::Scope;
use super::value::Value;

pub fn run_expression(expr: &ExpressionNode, scope: &Scope) -> Result<Value, String> {
    match expr {
        ExpressionNode::BinaryOperation { .. } => todo!(),
        ExpressionNode::Term(term) => match term {
            TermNode::Variable(var) => {
                scope.clone_variable(var).ok_or(format!("No such variable {var}"))
            },
            TermNode::Boolean(x) => Ok(Value::Bool(*x)),
            TermNode::Integer(x) => Ok(Value::Int(*x)),
            TermNode::String(x) => Ok(Value::String(x.clone())),
        }
    }
}
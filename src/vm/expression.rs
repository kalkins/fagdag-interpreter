use crate::parser::ast::{ExpressionNode, TermNode};
use super::scope::Scope;
use super::value::Value;

pub fn run_expression(expr: &ExpressionNode, scope: &Scope) -> Result<Value, String> {
    match expr {
        ExpressionNode::BinaryOperation { verb, lhs, rhs } => {
            todo!()
        },
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

#[cfg(test)]
mod test {
    use crate::parser::ast::{BinaryVerb, ExpressionNode, TermNode};
    use crate::vm::value::Value;
    use super::run_expression;
    use super::super::scope::Scope;

    #[test]
    fn test_addition() {
        let scope = Scope::new();

        let result = run_expression(
            &ExpressionNode::BinaryOperation {
                verb: BinaryVerb::Plus,
                lhs: ExpressionNode::Term(
                    TermNode::Integer(3)
                ).into(),
                rhs: ExpressionNode::Term(
                    TermNode::Integer(1)
                ).into(),
            },
            &scope,
        );

        assert_eq!(result, Ok(Value::Int(4)));
    }

    #[test]
    fn test_variable() {
        let mut scope = Scope::new();
        scope.add_variable("x", Value::Int(9));

        let result = run_expression(
            &ExpressionNode::Term(
                TermNode::Variable("x".into())
            ),
            &scope,
        );

        assert_eq!(result, Ok(Value::Int(9)));
    }
}

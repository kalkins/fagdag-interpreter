use crate::parser::ast::{ExpressionNode, TermNode, BinaryVerb};
use super::scope::Scope;
use super::value::Value;

pub fn run_expression(expr: &ExpressionNode, scope: &Scope) -> Result<Value, String> {
    match expr {
        ExpressionNode::BinaryOperation { verb, lhs, rhs } => {
            let lhs = run_expression(lhs, scope)?;
            let rhs = run_expression(rhs, scope)?;
            
            if lhs.ast_type() != rhs.ast_type() {
                Err(format!("Invalid operand types {} and {}", lhs.ast_type(), rhs.ast_type()))
            } else {
                match verb {
                    BinaryVerb::Plus => {
                        lhs + rhs
                    },
                    BinaryVerb::Minus => {
                        lhs - rhs
                    },
                    BinaryVerb::Compare => {
                        lhs.compare(&rhs)
                    }
                }
            }

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
    fn test_addition_int() {
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
    fn test_addition_string() {
        let scope = Scope::new();
        let result = run_expression(
            &ExpressionNode::BinaryOperation { 
                verb: BinaryVerb::Plus, 
                lhs: ExpressionNode::Term(
                    TermNode::String("Hello".to_string())
                ).into(), 
                rhs: ExpressionNode::Term(
                    TermNode::String("World".to_string())
                ).into()
            }, 
            &scope
        );

        assert_eq!(result, Ok(Value::String("HelloWorld".to_string())));
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

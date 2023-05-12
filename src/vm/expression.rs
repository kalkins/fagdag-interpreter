use crate::parser::ast::{ExpressionNode, TermNode, BinaryVerb};
use super::scope::Scope;
use super::value::Value;
use crate::vm::run_function;
use itertools::Itertools;

pub fn run_expression(expr: &ExpressionNode, scope: &Scope) -> Result<Value, String> {
    match expr {
        ExpressionNode::BinaryOperation { verb, lhs, rhs } => {
            let lv = run_expression(lhs, scope)?;
            let rv = run_expression(rhs, scope)?;

            match verb {
                BinaryVerb::Plus => {
                    match (lv, rv) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                        _ => Err("not ints".into())
                    }
                },
                BinaryVerb::Minus => {
                    match (lv, rv) {
                        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                        _ => Err("not ints".into())
                    }
                }

                BinaryVerb::Compare => {
                    Ok(Value::Bool(lv == rv))
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
            TermNode::FnCall(name, params) => {
                if let Some(fn_node) = scope.get_function(name) {
                    let mut new_scope = Scope::new();
                    let (params, errs) : (Vec<_>, Vec<_>) = params.iter().map(|el| run_expression(el, &scope)).into_iter().partition_result();

                    if errs.is_empty() {
                        match run_function(&fn_node, &new_scope, params) {
                            Ok(Some(value)) => Ok(value),
                            Ok(None) => Ok(Value::Bool(true)),
                            Err(error) => Err(error)
                        }
                    }
                    else {
                        Err(errs.join("\n"))
                    }
                }
                else {
                    Err("Function does not exist!".into())
                }
            }
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

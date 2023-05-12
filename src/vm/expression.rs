use crate::parser::ast::{BinaryVerb, ExpressionNode, TermNode};
use super::scope::Scope;
use super::value::Value;

pub fn run_expression(expr: &ExpressionNode, scope: &Scope) -> Result<Value, String> {
    match expr {
        ExpressionNode::BinaryOperation { verb, lhs, rhs } => {
            let lhs = run_expression(lhs, scope)?;
            let rhs = run_expression(rhs, scope)?;

            match verb {
                BinaryVerb::Plus => {
                    match (&lhs, &rhs) {
                        (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x+y)),
                        (Value::String(x), Value::Int(y)) => {
                            let lhs = x.clone();
                            let rhs = y.to_string();
                            Ok(Value::String(lhs + &rhs))
                        },
                        (Value::Int(x), Value::String(y)) => {
                            let lhs = x;
                            let rhs = y.parse::<i32>().map_err(|e| format!("Can't convert {} to string, to add to {}", y, x))?;
                            Ok(Value::Int(lhs + rhs))
                        },
                        (Value::String(x), Value::String(y)) => Ok(Value::String(x.clone() + y)),
                        _ => Err(format!("Can not add {} and {} as they are not both ints", lhs, rhs))
                    }
                }
                BinaryVerb::Minus => {
                    match (&lhs, &rhs) {
                        (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x-y)),
                        _ => Err(format!("Can not add {} and {} as they are not both ints", lhs, rhs))
                    }
                }
                BinaryVerb::Compare => {
                    Ok(Value::Bool(
                        lhs.str_repr() == rhs.str_repr()
                    ))
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
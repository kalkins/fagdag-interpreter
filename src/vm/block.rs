use crate::parser::ast::BlockNode;
use crate::vm::expression::run_expression;
use crate::vm::scope::Scope;
use crate::vm::value::Value;

pub fn run_block(block: &Vec<BlockNode>, parent: &Scope) -> Result<Option<Value>, String> {
    let mut scope = Scope::from_parent(parent);
    let mut return_value = None;

    for node in block {
        match node {
            BlockNode::VariableDefinition { name, type_name, value } => {
                if scope.get_variable(name).is_none() {
                    let v = run_expression(value, &scope)?;
                    if v.ast_type() == *type_name {
                        scope.add_variable(name, v);
                    }
                    else {
                        Err(format!("Invalid variable definition. Mismatch types {} and {}", v.ast_type(), type_name))?;
                    }
                }
                else {
                    Err(format!("Definition of {} shadows previously declared variable", name))?;
                }

            },
            BlockNode::Assignment { lhs, rhs } => {
                let v = scope.get_variable(lhs).ok_or("Hello".to_string())?;
                let e = run_expression(rhs, &scope)?;
                if e.ast_type() == v.ast_type() {
                    scope.add_variable(lhs, e);
                }
                else {
                    Err(format!("Mismatch in assigment"))?;
                }
            },
            BlockNode::Expression(expr) => {
                run_expression(expr, &scope)?;
            },
            BlockNode::Block(nested) => {
                match run_block(nested, &scope)? {
                    Some(x) => {
                        return_value = Some(x);
                        break;
                    },
                    None => {}
                }
            },
            BlockNode::Return(expr) => {
                return_value = Some(run_expression(expr, &scope)?);
                break;
            }
        };
    }

    Ok(return_value)
}

#[cfg(test)]
mod test {
    use crate::parser::ast::{BlockNode, ExpressionNode, TermNode, Type};
    use crate::vm::block::run_block;
    use crate::vm::scope::Scope;
    use crate::vm::value::Value;

    #[test]
    fn test_variable_definition() {
        let scope = Scope::new();

        let result = run_block(
            &vec![
                BlockNode::VariableDefinition {
                    name: "x".into(),
                    type_name: Type::Int,
                    value: ExpressionNode::Term(TermNode::Integer(5)),
                },
                BlockNode::Return(
                    ExpressionNode::Term(TermNode::Variable("x".into()))
                )
            ],
            &scope,
        ).expect("Error while defining simple variable");

        assert_eq!(result, Some(Value::Int(5)));

        run_block(
            &vec![
                BlockNode::VariableDefinition {
                    name: "x".into(),
                    type_name: Type::Int,
                    value: ExpressionNode::Term(TermNode::String("test".into())),
                },
                BlockNode::Return(
                    ExpressionNode::Term(TermNode::Variable("x".into()))
                )
            ],
            &scope,
        ).expect_err("The value of the variable should be required to be int");
    }

    #[test]
    fn test_variable_assignment() {
        let scope = Scope::new();

        let result = run_block(
            &vec![
                BlockNode::VariableDefinition {
                    name: "x".into(),
                    type_name: Type::Int,
                    value: ExpressionNode::Term(TermNode::Integer(3)),
                },
                BlockNode::Assignment {
                    lhs: "x".into(),
                    rhs: ExpressionNode::Term(TermNode::Integer(5)),
                },
                BlockNode::Return(
                    ExpressionNode::Term(TermNode::Variable("x".into()))
                )
            ],
            &scope,
        ).expect("Error with assignment");

        assert_eq!(result, Some(Value::Int(5)));
    }

    #[test]
    fn test_simple_nested_block() {
        let scope = Scope::new();

        let result = run_block(
            &vec![
                BlockNode::Block(vec![]),
            ],
            &scope,
        ).expect("Error with nested block");

        assert_eq!(result, None)
    }

    #[test]
    fn test_complex_nested_block() {
        let scope = Scope::new();

        let result = run_block(
            &vec![
                BlockNode::VariableDefinition {
                    name: "x".into(),
                    type_name: Type::Int,
                    value: ExpressionNode::Term(TermNode::Integer(3)),
                },
                BlockNode::Block(vec![
                    BlockNode::VariableDefinition {
                        name: "y".into(),
                        type_name: Type::Int,
                        value: ExpressionNode::Term(TermNode::Variable("x".to_string())),
                    },
                ])
            ],
            &scope,
        ).expect("Error with nested block");

        assert_eq!(result, None)
    }

    #[test]
    fn test_return_from_nested_block() {
        let scope = Scope::new();

        let result = run_block(
            &vec![
                BlockNode::Block(vec![
                    BlockNode::Return(
                        ExpressionNode::Term(TermNode::Integer(1))
                    )
                ]),
                BlockNode::Return(
                    ExpressionNode::Term(TermNode::Integer(2))
                )
            ],
            &scope,
        ).expect("Error with nested block");

        assert_eq!(result, Some(Value::Int(1)))
    }
}

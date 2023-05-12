use crate::parser::ast::{BlockNode, FunctionNode};
use super::expression::run_expression;
use super::scope::Scope;
use super::value::Value;

pub fn run_function(function: &FunctionNode, parent: &Scope, args: Vec<Value>) -> Result<Option<Value>, String> {
    let mut scope = Scope::from_parent(parent);

    if function.parameters.len() != args.len() {
        Err(format!("Function {} expected {} arguments, got {}", function.name, function.parameters.len(), args.len()))
    } else {
        // Handle parameters
        for (param, value) in function.parameters.iter().zip(args) {
            if value.ast_type() != param.type_name {
                Err(format!(
                    "Parameter {} to function {} should be of type {}, but is {}",
                    param.name,
                    function.name,
                    param.type_name,
                    value,
                ))?
            } else {
                scope.add_variable(&param.name, value);
            }
        }

        let mut return_value = None;

        // Run through each statement
        for node in &function.block {
            match node {
                BlockNode::VariableDefinition { name, type_name, value } => {
                    todo!()
                },
                BlockNode::Assignment { lhs, rhs } => {
                   todo!()
                },
                BlockNode::Expression(expr) => run_expression(expr, &scope)?,
                BlockNode::Return(expr) => {
                    return_value = Some(run_expression(expr, &scope)?);
                    break;
                }
            };
        }

        // Handle the return value
        if let Some(value) = return_value {
            if let Some(return_type) = &function.return_type {
                if value.ast_type() == *return_type {
                    Ok(Some(value))
                } else {
                    Err(format!("Function {} returned {}, but the return type is {}", function.name, value, return_type))
                }
            } else {
                Err(format!("Function {} returned {}, but it doesn't have a return value", function.name, value))
            }
        } else if function.return_type.is_none() {
            Ok(None)
        } else {
            Err(format!("Expected return from function {}", function.name))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::ast::*;
    use crate::vm::value::Value;
    use super::run_function;
    use super::super::{
        scope::Scope,
    };

    #[test]
    fn test_empty_function() {
        let scope = Scope::new();

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![],
                return_type: None,
                block: vec![],
            },
            &scope,
            vec![],
        ).expect("An empty function should be allowed");
    }

    #[test]
    fn test_parameter_list() {
        let scope = Scope::new();

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![
                    FunctionParam { name: "x".into(), type_name: Type::Int },
                ],
                return_type: None,
                block: vec![],
            },
            &scope,
            vec![
                Value::Int(0),
            ],
        ).expect("Arguments matching parameter list should be allowed");

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![
                    FunctionParam { name: "x".into(), type_name: Type::Int },
                    FunctionParam { name: "y".into(), type_name: Type::String },
                ],
                return_type: None,
                block: vec![],
            },
            &scope,
            vec![
                Value::Int(0),
                Value::String("test".into())
            ],
        ).expect("Arguments matching parameter list should be allowed");

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![
                    FunctionParam { name: "x".into(), type_name: Type::Int },
                ],
                return_type: None,
                block: vec![],
            },
            &scope,
            vec![
                Value::Int(0),
                Value::String("test".into())
            ],
        ).expect_err("Extra arguments should not be allowed");

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![
                    FunctionParam { name: "x".into(), type_name: Type::Int },
                    FunctionParam { name: "y".into(), type_name: Type::String },
                ],
                return_type: None,
                block: vec![],
            },
            &scope,
            vec![
                Value::Int(0),
            ],
        ).expect_err("Too few arguments should not be allowed");

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![
                    FunctionParam { name: "x".into(), type_name: Type::Int },
                ],
                return_type: None,
                block: vec![],
            },
            &scope,
            vec![
                Value::String("".into()),
            ],
        ).expect_err("Arguments should be required to be of the correct type");
    }

    #[test]
    fn test_return_value_match_return_type() {
        let scope = Scope::new();

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![],
                return_type: Some(Type::Bool),
                block: vec![
                    BlockNode::Return(
                        ExpressionNode::Term(TermNode::Boolean(true))
                    )
                ],
            },
            &scope,
            vec![],
        ).expect("Function does not accept correct return value");

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![],
                return_type: Some(Type::String),
                block: vec![
                    BlockNode::Return(
                        ExpressionNode::Term(TermNode::Boolean(true))
                    )
                ],
            },
            &scope,
            vec![],
        ).expect_err("A function should be required to return the designated return type");

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![],
                return_type: None,
                block: vec![
                    BlockNode::Return(
                        ExpressionNode::Term(TermNode::Boolean(true))
                    )
                ],
            },
            &scope,
            vec![],
        ).expect_err("A function should be prohibited from returning a value when no return type is specified");

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![],
                return_type: Some(Type::Int),
                block: vec![],
            },
            &scope,
            vec![],
        ).expect_err("A function should be prohibited from not returning a value when a return type is specified");
    }

    #[test]
    fn test_variable_definition() {
        let scope = Scope::new();

        let result = run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![],
                return_type: Some(Type::Bool),
                block: vec![
                    BlockNode::VariableDefinition {
                        name: "x".into(),
                        type_name: Type::Int,
                        value: ExpressionNode::Term(TermNode::Integer(5)),
                    },
                    BlockNode::Return(
                        ExpressionNode::Term(TermNode::Variable("x".into()))
                    )
                ],
            },
            &scope,
            vec![],
        ).expect("Error while defining simple variable");

        assert_eq!(result, Some(Value::Int(5)));

        run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![],
                return_type: Some(Type::String),
                block: vec![
                    BlockNode::VariableDefinition {
                        name: "x".into(),
                        type_name: Type::Int,
                        value: ExpressionNode::Term(TermNode::String("test".into())),
                    },
                    BlockNode::Return(
                        ExpressionNode::Term(TermNode::Variable("x".into()))
                    )
                ],
            },
            &scope,
            vec![],
        ).expect_err("The value of the variable should be required to be int");
    }

    #[test]
    fn test_variable_assignment() {
        let scope = Scope::new();

        let result = run_function(
            &FunctionNode {
                name: "test".into(),
                parameters: vec![],
                return_type: Some(Type::Int),
                block: vec![
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
            },
            &scope,
            vec![],
        ).expect("Error with assignment");

        assert_eq!(result, Some(Value::Int(5)));
    }
}

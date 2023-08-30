use crate::parser::ast::BlockNode;
use crate::vm::expression::run_expression;
use crate::vm::scope::Scope;
use crate::vm::value::Value;

pub fn run_block(block: &Vec<BlockNode>, scope: &mut Scope) -> Result<Option<Value>, String> {
    let mut return_value = None;

    for node in block {
        match node {
            BlockNode::VariableDefinition { name, type_name, value } => {
                if scope.get_variable(name).is_none() {
                    let v = run_expression(value, scope)?;
                    if v.ast_type() == *type_name {
                        scope.add_variable(name, v);
                    } else {
                        Err(format!("Invalid variable definition. Mismatch types {} and {}", v.ast_type(), type_name))?;
                    }
                } else {
                    Err(format!("Definition of {} shadows previously declared variable", name))?;
                }

            },
            BlockNode::Assignment { lhs, rhs } => {
                let v = scope.get_variable(lhs).ok_or("Hello".to_string())?;
                let e = run_expression(rhs, scope)?;

                if e.ast_type() == v.ast_type() {
                    scope.set_variable(lhs, e)?;
                } else {
                    Err("Mismatch in assigment".to_string())?;
                }
            },
            BlockNode::Expression(expr) => {
                run_expression(expr, &scope)?;
            },
            BlockNode::IfStatement { condition, block } => {
                match run_expression(condition, &scope)? {
                    Value::Bool(result) => {
                        if result {
                            return_value = scope.subscope(|s| run_block(block, s))?
                        }
                    }
                    x => {
                        Err(format!("Invalid condition result {x} in if-statement"))?
                    }
                }
            }
            BlockNode::Block(nested) => {
                return_value = scope.subscope(|s| run_block(nested, s))?;
            },
            BlockNode::Return(expr) => {
                return_value = Some(
                    scope.subscope(|s| run_expression(expr, s))?
                );
            }
        };

        if return_value.is_some() {
            break;
        }
    }

    Ok(return_value)
}

#[cfg(test)]
mod test {
    use crate::parser::ast::{BinaryVerb, BlockNode, ExpressionNode, TermNode, Type};
    use crate::vm::block::run_block;
    use crate::vm::scope::Scope;
    use crate::vm::value::Value;

    #[test]
    fn test_variable_definition() {
        let mut scope = Scope::new();

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
            &mut scope,
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
            &mut scope,
        ).expect_err("The value of the variable should be required to be int");
    }

    #[test]
    fn test_variable_assignment() {
        let mut scope = Scope::new();

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
            &mut scope,
        ).expect("Error with assignment");

        assert_eq!(result, Some(Value::Int(5)));
    }

    #[test]
    fn test_simple_nested_block() {
        let mut scope = Scope::new();

        let result = run_block(
            &vec![
                BlockNode::Block(vec![]),
            ],
            &mut scope,
        ).expect("Error with nested block");

        assert_eq!(result, None)
    }

    #[test]
    fn test_complex_nested_block() {
        let mut scope = Scope::new();

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
            &mut scope,
        ).expect("Error with nested block");

        assert_eq!(result, None)
    }

    #[test]
    fn test_return_from_nested_block() {
        let mut scope = Scope::new();

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
            &mut scope,
        ).expect("Error with nested block");

        assert_eq!(result, Some(Value::Int(1)))
    }

    #[test]
    fn test_if_statement() {
        let mut scope = Scope::new();

        let result = run_block(
            &vec![
                BlockNode::IfStatement {
                    condition: ExpressionNode::Term(TermNode::Boolean(true)),
                    block: vec![
                        BlockNode::Return(
                            ExpressionNode::Term(TermNode::Integer(2))
                        ),
                    ],
                },
                BlockNode::Return(
                    ExpressionNode::Term(TermNode::Integer(0))
                ),
            ],
            &mut scope,
        ).expect("Error with nested block");

        assert_eq!(result, Some(Value::Int(2)))
    }

    #[test]
    fn test_complex_if() {
        let mut scope = Scope::new();

        let result = run_block(
            &vec![
                BlockNode::VariableDefinition {
                    name: "x".into(),
                    type_name: Type::Int,
                    value: ExpressionNode::Term(
                        TermNode::Integer(0)
                    ),
                },
                BlockNode::IfStatement {
                    condition: ExpressionNode::BinaryOperation {
                        verb: BinaryVerb::Compare,
                        lhs: Box::new(ExpressionNode::Term(TermNode::Variable("x".into()))),
                        rhs: Box::new(ExpressionNode::Term(TermNode::Integer(0))),
                    },
                    block: vec![
                        BlockNode::Assignment {
                            lhs: "x".into(),
                            rhs: ExpressionNode::Term(TermNode::Integer(5)),
                        }
                    ],
                },
                BlockNode::Return(
                    ExpressionNode::Term(TermNode::Variable("x".into()))
                ),
            ],
            &mut scope,
        ).expect("Error with nested block");

        assert_eq!(result, Some(Value::Int(5)))
    }
}

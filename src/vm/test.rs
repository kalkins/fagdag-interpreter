use crate::parser::ast::*;
use super::super::run;

#[test]
fn test_smallest_program() {
    let result = run(
        &Program {
            nodes: vec![
                FunctionNode {
                    name: "main".into(),
                    parameters: vec![],
                    return_type: Some(Type::Int),
                    block: vec![
                        BlockNode::Return(
                            ExpressionNode::Term(
                                TermNode::Integer(5)
                            )
                        )
                    ],
                }
            ]
        }
    );

    assert_eq!(result, Ok(5))
}

#[test]
fn test_missing_main() {
    run(
        &Program {
            nodes: vec![]
        }
    ).expect_err("A main function should be required");

    run(
        &Program {
            nodes: vec![
                FunctionNode {
                    name: "test".into(),
                    parameters: vec![],
                    return_type: None,
                    block: vec![],
                }
            ]
        }
    ).expect_err("The main function should be required to have the name 'main'");
}

#[test]
fn test_main_return_type() {
    run(
        &Program {
            nodes: vec![
                FunctionNode {
                    name: "main".into(),
                    parameters: vec![],
                    return_type: Some(Type::String),
                    block: vec![
                        BlockNode::Return(
                            ExpressionNode::Term(
                                TermNode::String("test".into())
                            )
                        )
                    ],
                }
            ]
        }
    ).expect_err("The main function should be required to return int");

    run(
        &Program {
            nodes: vec![
                FunctionNode {
                    name: "main".into(),
                    parameters: vec![],
                    return_type: None,
                    block: vec![],
                }
            ]
        }
    ).expect_err("The main function should be required to return int");
}

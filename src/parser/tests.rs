use crate::parser::ast::*;
use crate::parser::parse;

// Move error handling away from each individual test
fn parse_helper(input: &str) -> Program {
    parse(input).unwrap_or_else(|e| {
        panic!("{e}");
    })
}

#[test]
fn test_empty_program() {
    let program = parse_helper("");

    assert_eq!(program.nodes.len(), 0);
}

#[test]
fn test_function_simple() {
    let ast = parse_helper("function test() {}");

    let expected = RootNode::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: None,
        block: vec![],
    };

    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0], expected);
}

#[test]
fn test_function_params() {
    let ast = parse_helper("function test(x: int, y: string) {}");

    let expected = RootNode::Function {
        name: "test".into(),
        parameters: vec![
            FunctionParam {
                name: "x".into(),
                type_name: Type::Int,
            },
            FunctionParam {
                name: "y".into(),
                type_name: Type::String,
            }
        ],
        return_type: None,
        block: vec![],
    };

    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0], expected);
}

#[test]
fn test_function_return_type() {
    let ast = parse_helper("function test() -> string {}");

    let expected = RootNode::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: Some(Type::String),
        block: vec![],
    };

    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0], expected);
}

#[test]
fn test_variable_def() {
    let ast = parse_helper("
        function test() {
            var x: int = 5;
        }
    ");

    let expected = RootNode::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: None,
        block: vec![
            BlockNode::VariableDefinition {
                name: "x".into(),
                type_name: Type::Int,
                value: ExpressionNode::Term(
                    TermNode::Integer(5)
                )
            }
        ],
    };

    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0], expected);
}

#[test]
fn test_assignment() {
    let ast = parse_helper("
        function test() {
            var x: int = 5;
            x = 9;
        }
    ");

    let expected = RootNode::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: None,
        block: vec![
            BlockNode::VariableDefinition {
                name: "x".into(),
                type_name: Type::Int,
                value: ExpressionNode::Term(
                    TermNode::Integer(5)
                )
            },
            BlockNode::Assignment {
                lhs: "x".into(),
                rhs: ExpressionNode::Term(
                    TermNode::Integer(9)
                )
            }
        ],
    };

    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0], expected);
}

#[test]
fn test_expr_addition() {
    let ast = parse_helper("
        function test() {
            var x: int = +5;
            x = (9 + x) + 8;
        }
    ");

    let expected = RootNode::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: None,
        block: vec![
            BlockNode::VariableDefinition {
                name: "x".into(),
                type_name: Type::Int,
                value: ExpressionNode::Term(
                    TermNode::Integer(5)
                )
            },
            BlockNode::Assignment {
                lhs: "x".into(),
                rhs: ExpressionNode::BinaryOperation {
                    verb: BinaryVerb::Plus,
                    lhs: ExpressionNode::BinaryOperation {
                        verb: BinaryVerb::Plus,
                        lhs: ExpressionNode::Term(
                            TermNode::Integer(9)
                        ).into(),
                        rhs: ExpressionNode::Term(
                            TermNode::Variable("x".into())
                        ).into(),
                    }.into(),
                    rhs: ExpressionNode::Term(
                        TermNode::Integer(8)
                    ).into()
                }
            }
        ],
    };

    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0], expected);
}

#[test]
fn test_negative_int() {
    let ast = parse_helper("
        function test() {
            var x: int = -5;
            x = -4 - -2;
        }
    ");

    let expected = RootNode::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: None,
        block: vec![
            BlockNode::VariableDefinition {
                name: "x".into(),
                type_name: Type::Int,
                value: ExpressionNode::Term(
                    TermNode::Integer(-5)
                )
            },
            BlockNode::Assignment {
                lhs: "x".into(),
                rhs: ExpressionNode::BinaryOperation {
                    verb: BinaryVerb::Minus,
                    lhs: ExpressionNode::Term(
                        TermNode::Integer(-4)
                    ).into(),
                    rhs: ExpressionNode::Term(
                        TermNode::Integer(-2)
                    ).into(),
                }
            }
        ],
    };

    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0], expected);
}

#[test]
fn test_string() {
    let ast = parse_helper("
        function test() {
            var x: string = \"This is a test\";
        }
    ");

    let expected = RootNode::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: None,
        block: vec![
            BlockNode::VariableDefinition {
                name: "x".into(),
                type_name: Type::String,
                value: ExpressionNode::Term(
                    TermNode::String("This is a test".into())
                )
            },
        ],
    };

    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0], expected);
}

#[test]
fn test_bool() {
    let ast = parse_helper("
        function test() {
            var x: bool = true;
            var y: bool = false;
        }
    ");

    let expected = RootNode::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: None,
        block: vec![
            BlockNode::VariableDefinition {
                name: "x".into(),
                type_name: Type::Bool,
                value: ExpressionNode::Term(
                    TermNode::Boolean(true)
                )
            },
            BlockNode::VariableDefinition {
                name: "y".into(),
                type_name: Type::Bool,
                value: ExpressionNode::Term(
                    TermNode::Boolean(false)
                )
            },
        ],
    };

    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0], expected);
}

#[test]
fn test_return() {
    let ast = parse_helper("
        function test() {
            return 5;
        }
    ");

    let expected = RootNode::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: None,
        block: vec![
            BlockNode::Return(
                ExpressionNode::Term(
                    TermNode::Integer(5)
                )
            ),
        ],
    };

    assert_eq!(ast.nodes.len(), 1);
    assert_eq!(ast.nodes[0], expected);
}

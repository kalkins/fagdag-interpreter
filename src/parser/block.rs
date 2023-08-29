use pest::iterators::Pair;
use crate::parser::utils::parse_all;
use super::ast::BlockNode;
use super::error::ParseError;
use super::from_pest::FromPest;
use super::Rule;
use super::utils::parse_next;

impl FromPest<'_> for BlockNode {
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::def_var => {
                let mut inner = pair.clone().into_inner();
                let (name, type_name) = parse_next(&mut inner, &pair)?;

                Ok(
                    BlockNode::VariableDefinition {
                        name,
                        type_name,
                        value: parse_next(&mut inner, &pair)?,
                    }
                )
            },
            Rule::assignment => {
                let mut inner = pair.clone().into_inner();

                Ok(
                    BlockNode::Assignment {
                        lhs: parse_next(&mut inner, &pair)?,
                        rhs: parse_next(&mut inner, &pair)?,
                    }
                )
            },
            Rule::block => {
                let mut inner = pair.clone().into_inner();
                //Ok(BlockNode::Block(parse_next(&mut inner)?.unwrap_or(vec![])))
                Ok(BlockNode::Block(parse_all(&mut inner)?))
            },
            Rule::return_stmt => {
                let mut inner = pair.clone().into_inner();
                Ok(BlockNode::Return(parse_next(&mut inner, &pair)?))
            },
            Rule::expr_stmt => {
                let mut inner = pair.clone().into_inner();

                Ok(BlockNode::Expression(parse_next(&mut inner, &pair)?))
            },
            Rule::if_statement => {
                let mut inner = pair.clone().into_inner();

                Ok(
                    BlockNode::IfStatement {
                        condition: parse_next(&mut inner, &pair)?,
                        block: parse_next(&mut inner, &pair)?,
                    }
                )
            }
            rule => Err(ParseError::wrong_rule(&pair, rule))
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::{
        ast::*,
        test::helper::*,
    };

    #[test]
    fn test_variable_def() {
        let nodes = parse_block("var x: int = 5;");

        let expected = vec![
            BlockNode::VariableDefinition {
                name: "x".into(),
                type_name: Type::Int,
                value: ExpressionNode::Term(
                    TermNode::Integer(5)
                )
            }
        ];

        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_assignment() {
        let nodes = parse_block("
            var x: int = 5;
            x = 9;
        ");

        let expected = vec![
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
        ];

        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_return() {
        let nodes = parse_block("
            return 5;
        ");

        let expected = vec![
            BlockNode::Return(
                ExpressionNode::Term(
                    TermNode::Integer(5)
                )
            ),
        ];

        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_nested() {
        let nodes = parse_block("
            {
            }
        ");

        let expected = vec![
            BlockNode::Block(vec![]),
        ];

        assert_eq!(nodes, expected);

        let nodes = parse_block("
            var x: int = 5;
            {
                var y: int = 10;
                {
                    x = y;
                }

                return x;
            }
        ");

        let expected = vec![
            BlockNode::VariableDefinition {
                name: "x".into(),
                type_name: Type::Int,
                value: ExpressionNode::Term(
                    TermNode::Integer(5)
                )
            },

            BlockNode::Block(vec![
                BlockNode::VariableDefinition {
                    name: "y".into(),
                    type_name: Type::Int,
                    value: ExpressionNode::Term(
                        TermNode::Integer(10)
                    )
                },

                BlockNode::Block(vec![
                    BlockNode::Assignment {
                        lhs: "x".into(),
                        rhs: ExpressionNode::Term(
                            TermNode::Variable("y".into())
                        )
                    }
                ]),

                BlockNode::Return(
                    ExpressionNode::Term(
                        TermNode::Variable("x".into())
                    )
                ),
            ]),
        ];

        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_if_statement() {
        let nodes = parse_block("
            var x: int = 5;
            if (x == 5) {
                return x;
            }
        ");

        let expected = vec![
            BlockNode::VariableDefinition {
                name: "x".into(),
                type_name: Type::Int,
                value: ExpressionNode::Term(
                    TermNode::Integer(5)
                )
            },

            BlockNode::IfStatement {
                condition: ExpressionNode::BinaryOperation {
                    verb: BinaryVerb::Compare,
                    lhs: ExpressionNode::Term(TermNode::Variable("x".to_string())).into(),
                    rhs: ExpressionNode::Term(TermNode::Integer(5)).into(),
                },
                block: vec![
                    BlockNode::Return(
                        ExpressionNode::Term(
                            TermNode::Variable("x".into())
                        )
                    ),
                ],
            },
        ];

        assert_eq!(nodes, expected);
    }
}

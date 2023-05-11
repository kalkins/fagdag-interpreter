use pest::iterators::Pair;
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
            Rule::return_stmt => {
                let mut inner = pair.clone().into_inner();
                Ok(BlockNode::Return(parse_next(&mut inner, &pair)?))
            },
            Rule::expr_stmt => {
                let mut inner = pair.clone().into_inner();

                Ok(BlockNode::Expression(parse_next(&mut inner, &pair)?))
            }
            rule => Err(ParseError::wrong_rule(&pair, rule))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::ast::*;
    use crate::parser::tests::helper::*;

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
}
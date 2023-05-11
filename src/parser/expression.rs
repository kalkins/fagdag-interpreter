use pest::iterators::Pair;
use super::ast::{BinaryVerb, ExpressionNode, TermNode};
use super::error::ParseError;
use super::from_pest::{FromPest, ParsePest};
use super::Rule;
use super::utils::{map_next, parse_next};

impl FromPest<'_> for ExpressionNode {
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::binary_op => {
                let mut inner = pair.clone().into_inner();

                Ok(
                    ExpressionNode::BinaryOperation {
                        lhs: parse_next(&mut inner, &pair)?,
                        verb: parse_next(&mut inner, &pair)?,
                        rhs: parse_next(&mut inner, &pair)?,
                    }
                )
            }
            Rule::name => {
                Ok(
                    ExpressionNode::Term(
                        TermNode::Variable(pair.parse()?)
                    )
                )
            }
            Rule::bool => {
                Ok(
                    ExpressionNode::Term(
                        TermNode::Boolean(
                            match pair.as_str() {
                                "true" => Ok(true),
                                "false" => Ok(false),
                                str => Err(ParseError::from_pair(&pair, format!("Invalid boolean {str}")))
                            }?
                        )
                    )
                )
            }
            Rule::str => {
                let mut inner = pair.clone().into_inner();

                Ok(
                    ExpressionNode::Term(
                        TermNode::String(
                            // Get the inner text to avoid the literal "s
                            map_next(&mut inner, &pair, |p| p.as_str().to_string())?
                        )
                    )
                )
            }
            Rule::int => {
                Ok(
                    ExpressionNode::Term(
                        TermNode::Integer(
                            pair.as_str().parse().map_err(|_|
                                ParseError::from_pair(&pair, format!("Can't convert {} to int", pair.as_str()))
                            )?
                        )
                    )
                )
            }
            rule => Err(ParseError::wrong_rule(&pair, rule))
        }
    }
}

impl FromPest<'_> for BinaryVerb {
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::add => Ok(BinaryVerb::Plus),
            Rule::subtract => Ok(BinaryVerb::Minus),
            Rule::compare => Ok(BinaryVerb::Compare),
            rule => Err(ParseError::wrong_rule(&pair, rule))
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::{
        ast::*,
        tests::helper::*,
    };

    #[test]
    fn test_expr_addition() {
        let nodes = parse_block("
            +5;
            (9 + x) + 8;
        ");

        let expected = vec![
            BlockNode::Expression(
                ExpressionNode::Term(
                    TermNode::Integer(5)
                )
            ),
            BlockNode::Expression (
                ExpressionNode::BinaryOperation {
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
            ),
        ];

        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_negative_int() {
        let nodes = parse_block("
            -5;
            -4 - -2;
        ");

        let expected = vec![
            BlockNode::Expression(
                ExpressionNode::Term(
                    TermNode::Integer(-5)
                )
            ),
            BlockNode::Expression(
                ExpressionNode::BinaryOperation {
                    verb: BinaryVerb::Minus,
                    lhs: ExpressionNode::Term(
                        TermNode::Integer(-4)
                    ).into(),
                    rhs: ExpressionNode::Term(
                        TermNode::Integer(-2)
                    ).into(),
                }
            ),
        ];

        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_string() {
        let nodes = parse_block("
            \"This is a test\";
        ");

        let expected = vec![
            BlockNode::Expression(
                ExpressionNode::Term(
                    TermNode::String("This is a test".into())
                )
            ),
        ];

        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_bool() {
        let nodes = parse_block("
            true;
            false;
        ");

        let expected = vec![
            BlockNode::Expression(
                ExpressionNode::Term(
                    TermNode::Boolean(true)
                )
            ),
            BlockNode::Expression(
                ExpressionNode::Term(
                    TermNode::Boolean(false)
                )
            ),
        ];

        assert_eq!(nodes, expected);
    }
}
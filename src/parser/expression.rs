use pest::iterators::Pair;
use crate::parser::ast::{BinaryVerb, ExpressionNode, TermNode};
use crate::parser::error::ParseError;
use crate::parser::from_pest::{FromPest, ParsePest};
use crate::parser::Rule;
use crate::parser::utils::{map_next, parse_next};

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

use pest::iterators::{Pair, Pairs};
use crate::parser::ast::{Program, RootNode};
use crate::parser::error::ParseError;
use crate::parser::from_pest::{FromPest, ParsePest};
use crate::parser::Rule;
use crate::parser::utils::{parse_next, parse_next_option};

impl TryFrom<&mut Pairs<'_, Rule>> for Program {
    type Error = ParseError;

    fn try_from(value: &mut Pairs<'_, Rule>) -> Result<Self, Self::Error> {

        Ok(
            Program {
                nodes: ParseError::merge(
                    value.filter_map(|p| match p.as_rule() {
                        Rule::EOI => None,
                        _ => Some(p.parse())
                    })
                )?
            }
        )
    }
}

impl FromPest<'_> for RootNode {
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::func => {
                let mut inner = pair.clone().into_inner();

                Ok(RootNode::Function {
                    name: parse_next(&mut inner, &pair)?,
                    parameters: parse_next(&mut inner, &pair)?,
                    return_type: parse_next_option(&mut inner)?,
                    block: parse_next(&mut inner, &pair)?,
                })
            }
            rule => Err(ParseError::wrong_rule(&pair, rule))
        }
    }
}


use pest::iterators::Pair;
use crate::parser::ast::BlockNode;
use crate::parser::error::ParseError;
use crate::parser::from_pest::FromPest;
use crate::parser::Rule;
use crate::parser::utils::parse_next;

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
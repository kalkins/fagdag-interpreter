use pest::iterators::Pair;
use super::ast::{Type, Ident};
use super::error::ParseError;
use super::from_pest::{FromPest, ParsePest};
use super::Rule;
use super::utils::parse_next;

impl FromPest<'_> for Type {
    #[track_caller]
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::type_name => {
                match pair.as_str() {
                    "int" => Ok(Type::Int),
                    "bool" => Ok(Type::Bool),
                    "string" => Ok(Type::String),
                    text => Err(ParseError::from_pair(&pair, format!("Invalid type {text}")))
                }
            }
            rule => Err(ParseError::wrong_rule(&pair, rule))
        }
    }
}

impl FromPest<'_> for Ident {
    #[track_caller]
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::name => {
                Ok(pair.as_str().into())
            }
            rule => Err(ParseError::wrong_rule(&pair, rule)),
        }
    }
}

impl FromPest<'_> for (Ident, Type) {
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::type_decl => {
                let mut inner = pair.clone().into_inner();

                Ok(
                    (
                        parse_next(&mut inner, &pair)?,
                        parse_next(&mut inner, &pair)?,
                    )
                )
            }
            rule => Err(ParseError::wrong_rule(&pair, rule))
        }
    }
}

impl<'pest, T: FromPest<'pest>> FromPest<'pest> for Box<T> {
    fn from_pest(pair: Pair<'pest, Rule>) -> Result<Self, ParseError> {
        Ok(Box::new(pair.parse()?))
    }
}

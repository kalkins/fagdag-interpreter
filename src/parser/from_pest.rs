use pest::iterators::Pair;
use crate::parser::utils::parse_all;
use super::error::ParseError;
use super::Rule;

pub trait FromPest<'pest>: Sized {
    fn from_pest(pair: Pair<'pest, Rule>) -> Result<Self, ParseError>;
}

pub trait ParsePest<T> {
    fn parse(self) -> Result<T, ParseError>;
}

impl<'pest, T: FromPest<'pest>> ParsePest<T> for Pair<'pest, Rule> {
    #[track_caller]
    fn parse(self) -> Result<T, ParseError> {
        T::from_pest(self)
    }
}

impl<'pest, T: FromPest<'pest>> FromPest<'pest> for Vec<T> {
    #[track_caller]
    fn from_pest(pair: Pair<'pest, Rule>) -> Result<Self, ParseError> {
        parse_all(&mut pair.into_inner())
    }
}

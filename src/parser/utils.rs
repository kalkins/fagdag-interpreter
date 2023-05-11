use pest::iterators::{Pair, Pairs};
use crate::parser::error::ParseError;
use crate::parser::from_pest::{FromPest, ParsePest};
use crate::parser::Rule;

#[track_caller]
pub fn parse_next<'pest, T: FromPest<'pest>>(inner: &mut Pairs<'pest, Rule>, parent: &Pair<'pest, Rule>) -> Result<T, ParseError> {
    inner.next().ok_or(ParseError::end(parent))?.parse()
}

#[track_caller]
pub fn map_next<'pest, T, F>(inner: &mut Pairs<'pest, Rule>, parent: &Pair<'pest, Rule>, f: F) -> Result<T, ParseError>
    where F: FnOnce(Pair<'pest, Rule>) -> T
{
    inner.next().ok_or(ParseError::end(parent)).map(f)
}

#[track_caller]
pub fn parse_next_option<'pest, T: FromPest<'pest>>(inner: &mut Pairs<'pest, Rule>) -> Result<Option<T>, ParseError> {
    match inner.peek().map(|p| p.parse()) {
        Some(Ok(value)) => Ok(Some(value)),
        Some(Err(ParseError::UnexpectedEnd {..} | ParseError::UnexpectedRule {..})) => Ok(None),
        Some(Err(error)) => Err(error),
        None => Ok(None),
    }
}
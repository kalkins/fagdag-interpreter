use pest::iterators::{Pair, Pairs};
use super::error::ParseError;
use super::from_pest::{FromPest, ParsePest};
use super::Rule;

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
        Some(Ok(value)) => {
            inner.next();
            Ok(Some(value))
        },
        Some(Err(ParseError::UnexpectedEnd {..} | ParseError::UnexpectedRule {..})) => Ok(None),
        Some(Err(error)) => Err(error),
        None => Ok(None),
    }
}

#[track_caller]
pub fn parse_all<'pest, T: FromPest<'pest>>(inner: &mut Pairs<'pest, Rule>) -> Result<Vec<T>, ParseError> {
    ParseError::merge(
        inner.map(|p| p.parse())
    )
}

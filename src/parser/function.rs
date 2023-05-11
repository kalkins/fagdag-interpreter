use pest::iterators::Pair;
use super::ast::FunctionParam;
use super::error::ParseError;
use super::from_pest::{FromPest, ParsePest};
use super::Rule;

impl FromPest<'_> for FunctionParam {
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        let (ident, type_name) = pair.parse()?;

        Ok(FunctionParam { name: ident, type_name })
    }
}

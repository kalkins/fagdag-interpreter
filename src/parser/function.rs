use pest::iterators::Pair;
use crate::parser::ast::FunctionParam;
use crate::parser::error::ParseError;
use crate::parser::from_pest::{FromPest, ParsePest};
use crate::parser::Rule;

impl FromPest<'_> for FunctionParam {
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        let (ident, type_name) = pair.parse()?;

        Ok(FunctionParam { name: ident, type_name })
    }
}

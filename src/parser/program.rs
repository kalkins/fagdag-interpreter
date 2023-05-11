use pest::iterators::Pairs;
use super::ast::Program;
use super::error::ParseError;
use super::from_pest::ParsePest;
use super::Rule;

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

#[cfg(test)]
mod test {
    use super::super::{
        test::helper::*,
    };

    #[test]
    fn test_empty_program() {
        let program = parse_helper("");

        assert_eq!(program.nodes.len(), 0);
    }

}

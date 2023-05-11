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

#[cfg(test)]
mod test {
    use crate::parser::ast::*;
    use crate::parser::tests::helper::parse_helper;

    #[test]
    fn test_empty_program() {
        let program = parse_helper("");

        assert_eq!(program.nodes.len(), 0);
    }

    #[test]
    fn test_function_simple() {
        let ast = parse_helper("function test() {}");

        let expected = RootNode::Function {
            name: "test".into(),
            parameters: vec![],
            return_type: None,
            block: vec![],
        };

        assert_eq!(ast.nodes.len(), 1);
        assert_eq!(ast.nodes[0], expected);
    }

    #[test]
    fn test_function_params() {
        let ast = parse_helper("function test(x: int, y: string) {}");

        let expected = RootNode::Function {
            name: "test".into(),
            parameters: vec![
                FunctionParam {
                    name: "x".into(),
                    type_name: Type::Int,
                },
                FunctionParam {
                    name: "y".into(),
                    type_name: Type::String,
                }
            ],
            return_type: None,
            block: vec![],
        };

        assert_eq!(ast.nodes.len(), 1);
        assert_eq!(ast.nodes[0], expected);
    }

    #[test]
    fn test_function_return_type() {
        let ast = parse_helper("function test() -> string {}");

        let expected = RootNode::Function {
            name: "test".into(),
            parameters: vec![],
            return_type: Some(Type::String),
            block: vec![],
        };

        assert_eq!(ast.nodes.len(), 1);
        assert_eq!(ast.nodes[0], expected);
    }
}

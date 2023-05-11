use pest::iterators::Pair;
use super::ast::{FunctionNode, FunctionParam};
use super::error::ParseError;
use super::from_pest::{FromPest, ParsePest};
use super::utils::{parse_next, parse_next_option};
use super::Rule;

impl FromPest<'_> for FunctionNode {
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::func => {
                let mut inner = pair.clone().into_inner();

                Ok(FunctionNode {
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

impl FromPest<'_> for FunctionParam {
    fn from_pest(pair: Pair<'_, Rule>) -> Result<Self, ParseError> {
        let (ident, type_name) = pair.parse()?;

        Ok(FunctionParam { name: ident, type_name })
    }
}

#[cfg(test)]
mod test {
    use super::super::{
        ast::*,
        test::helper::*,
    };

    #[test]
    fn test_function_simple() {
        let ast = parse_helper("function test() {}");

        let expected = FunctionNode {
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

        let expected = FunctionNode {
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

        let expected = FunctionNode {
            name: "test".into(),
            parameters: vec![],
            return_type: Some(Type::String),
            block: vec![],
        };

        assert_eq!(ast.nodes.len(), 1);
        assert_eq!(ast.nodes[0], expected);
    }

    #[test]
    fn test_function_return_type_with_block() {
        let ast = parse_helper("function test() -> string { return 5; }");

        let expected = FunctionNode {
            name: "test".into(),
            parameters: vec![],
            return_type: Some(Type::String),
            block: vec![
                BlockNode::Return(
                    ExpressionNode::Term(TermNode::Integer(5))
                )
            ],
        };

        assert_eq!(ast.nodes.len(), 1);
        assert_eq!(ast.nodes[0], expected);
    }
}

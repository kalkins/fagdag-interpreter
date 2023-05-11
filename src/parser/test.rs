pub mod helper {
    use super::super::{
        parse,
        ast::*,
    };

    /// Parse the input text to a program, and panic on errors.
    ///
    /// Useful for reducing the error handling necessary in each test.
    pub fn parse_helper(input: &str) -> Program {
        parse(input).unwrap_or_else(|e| {
            panic!("{e}");
        })
    }

    /// Parse a function body to a list of block nodes.
    ///
    /// Reduces the boilerplate of tests handing the function when they
    /// only care about the statements.
    pub fn parse_block(input: &str) -> Vec<BlockNode> {
        let mut nodes = parse_helper(
            &format!("
                function test() {{
                    {input}
                }}"
            )
        ).nodes;

        assert_eq!(nodes.len(), 1);

        if let Some(FunctionNode {block, .. }) = nodes.pop() {
            block
        } else {
            panic!()
        }
    }
}

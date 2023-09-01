mod function;
mod stack;
mod helper;
mod block;
mod expression;

use crate::compiler::function::compile_function;
use crate::parser::ast::Program;

pub fn compile(program: &Program) -> Result<String, String> {
    let preamble = include_str!("boot.s");

    let functions = program.nodes
        .iter()
        .map(compile_function)
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");

    Ok(
        format!(
            "{}\n# Generated code start\n\n{}",
            preamble,
            functions,
        )
    )
}
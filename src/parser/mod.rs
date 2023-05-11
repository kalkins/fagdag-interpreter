#[cfg(test)]
mod tests;
mod function;
mod common;
mod block;
mod expression;
mod ast;
mod rule;
mod from_pest;
mod program;
mod error;
mod utils;

use pest::Parser;
use pest_derive::Parser;
use self::ast::Program;

#[derive(Parser)]
#[grammar = "kwlang.pest"]
struct KWLangParser;

pub fn parse(input: &str) -> Result<Program, String> {
    match KWLangParser::parse(Rule::program, input) {
        Ok(mut pairs) => {
            #[cfg(debug_grammar)]
            println!("{}", pairs.to_json());

            Program::try_from(&mut pairs).map_err(|e| e.to_string())
        }
        Err(error) => {
            Err(format!("Error during parsing:\n{error}"))
        }
    }
}

extern crate pest;
extern crate pest_derive;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "kwlang.pest"]
struct KWLangParser;

pub fn parse(input: &str) {
    match KWLangParser::parse(Rule::program, input) {
        Ok(program) => {
            println!("{}", program.to_json());
        }
        Err(error) => {
            print!("Error during parsing:\n{}", error);
        }
    }
}

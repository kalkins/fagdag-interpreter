extern crate pest;
extern crate pest_derive;

use std::fs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "kwlang.pest"]
struct KWLangParser;

fn main() {
    let file = fs::read_to_string("tests/test.kw").expect("cannot read file");

    match KWLangParser::parse(Rule::program, &file) {
        Ok(program) => {
            println!("{}", program.to_json());
        }
        Err(error) => {
            print!("Error during parsing:\n{}", error);
        }
    }
}

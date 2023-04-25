mod parser;

use std::fs;
use crate::parser::parse;

fn main() {
    let file = fs::read_to_string("tests/test.kw").expect("cannot read file");
    parse(&file);
}

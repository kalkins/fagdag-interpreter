mod parser;

use std::{env, fs};
use std::io::{Read, stdin};
use std::process::exit;
use crate::parser::parse;

fn print_usage(prog: &str) {
    println!("USAGE: {prog} FILE")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = match &args.get(1) {
        Some(path) => {
            fs::read_to_string(path).expect("Cannot read file")
        }
        None => {
            print_usage(&args[0]);
            println!("No file provided");
            exit(-1);
        }
    };

    parse(&input);
}

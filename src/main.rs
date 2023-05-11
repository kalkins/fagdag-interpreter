mod parser;
mod vm;

extern crate pest_derive;
extern crate from_pest;
extern crate pest;

use std::{env, fs};
use std::process::exit;
use crate::parser::parse;
use crate::vm::run;

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

    match parse(&input) {
        Ok(program) => {
            println!("{program:#?}");

            match run(&program) {
                Ok(return_value) => println!("Program returned {return_value}"),
                Err(error) => println!("{error}")
            }
        }
        Err(error) => println!("{error}")
    }
}

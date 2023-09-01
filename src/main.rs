mod parser;
mod vm;
mod compiler;

extern crate pest_derive;
extern crate from_pest;
extern crate pest;

use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use crate::compiler::compile;
use crate::Operations::{Compile, Interpret};
use crate::parser::parse;
use crate::vm::run;

fn print_usage(prog: &str) {
    println!("\
USAGE: {prog} -i FILE
       {prog} -c FILE
")
}

enum Operations {
    Interpret,
    Compile,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let operation = match &args.get(1) {
        Some(arg) => {
            if *arg == "-i" {
                Interpret
            } else if *arg == "-c" {
                Compile
            } else {
                panic!("Invalid argument '{arg}'")
            }
        }
        None => {
            print_usage(&args[0]);
            println!("Argument required");
            exit(-1);
        }
    };

    if let Some(source_path) = &args.get(2) {
        let input = fs::read_to_string(source_path).expect("Cannot read file");

        match parse(&input) {
            Ok(program) => {
                println!("{program:#?}");

                match operation {
                    Interpret => {
                        match run(&program) {
                            Ok(return_value) => println!("Program returned {return_value}"),
                            Err(error) => println!("Error during execution:\n{error}")
                        }
                    }
                    Compile => {
                        match compile(&program) {
                            Ok(compiled) => {
                                println!("Compiled assembly:\n{compiled}");
                                let path = PathBuf::from(source_path);
                                let asm_path = path.with_extension("s");

                                fs::write(&asm_path, compiled).expect("Could not write assembly to file");

                                run_compiled(&asm_path);
                            },
                            Err(error) => println!("Error during compilation:\n{error}"),
                        }
                    }
                }
            }
            Err(error) => println!("{error}")
        }
    } else {
        print_usage(&args[0]);
        println!("No file provided");
        exit(-1);
    }
}

fn run_compiled(path: &Path) {
    let bin_path = path.with_extension("bin");

    let gcc = vec!["riscv64-elf-gcc"]
        .into_iter()
        .find(|c|
            Command::new("which")
                .arg(c)
                .output()
                .unwrap()
                .status
                .success()
        )
        .expect("Could not find gcc");

    println!("Running gcc");
    let gcc_result = Command::new(gcc)
        .args(["-g", "-Wl,-T,linker.lds", "-nostartfiles", "-nostdlib"])
        .arg(&path)
        .arg("-o")
        .arg(&bin_path)
        .status()
        .expect("Compilation failed");

    if gcc_result.success() {
        println!("Running qemu. Press 'Ctrl-A x' to abort\n");
        Command::new("qemu-system-riscv64")
            .args(["-machine", "virt", "-cpu", "rv64", "-smp", "1", "-m", "128M", "-nographic", "-serial", "mon:stdio", "-bios", "none"])
            .arg("-kernel")
            .arg(&bin_path)
            .status()
            .expect("Execution failed");
    } else {
        println!("Compilation failed");
        exit(-1);
    }
}
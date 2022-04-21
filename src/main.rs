mod compiler;
mod lexer;
mod lib;
mod repl;
mod tests;

use clap::Parser;
use compiler::compile;
use lexer::tokenize;
use lexer::tokens::Token;
use lib::utils::LocatedResult;
use repl::Repl;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

/// LCL programming language compiler and interactive shell
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Target file
    #[clap(parse(from_os_str))]
    input: Option<PathBuf>,

    /// Place the output into <OUTPUT>
    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn read_program(path: &str) -> LocatedResult<Vec<Token>> {
    let data = fs::read_to_string(path).expect("failed to read from file");
    tokenize(data.as_str(), path)
}

fn main() {
    let args = Args::parse();
    match args.input {
        Some(input) => {
            let mut program = match read_program(input.to_str().unwrap()) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            };
            let out = args.output.unwrap_or_else(|| {
                let mut default = PathBuf::new();
                default.set_file_name("output");
                default
            });
            if let Err(e) = compile(&mut program, out.to_str().unwrap()) {
                eprintln!("{}", e);
                exit(1);
            };
        }
        _ => {
            let mut repl = Repl::new(">> ");
            repl.run_loop();
        }
    }
}

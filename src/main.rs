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
use repl::REPL;
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
            // TODO: add output file support
            let program = match read_program(input.to_str().unwrap()) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            };
            if let Err(e) = compile(program) {
                eprintln!("{}", e);
                exit(1);
            };
        }
        _ => {
            let mut repl = REPL::new(">> ");
            repl.run_loop();
        }
    }
}

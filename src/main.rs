mod compiler;
mod lexer;
mod lib;
mod repl;
mod tests;

use compiler::compile;
use lexer::tokenize;
use lexer::tokens::Token;
use lib::utils::LocatedResult;
use repl::REPL;
use std::env;
use std::fs;
use std::process::exit;

fn read_program(path: &str) -> LocatedResult<Vec<Token>> {
    let data = fs::read_to_string(path).expect("failed to read from file");
    tokenize(data.as_str(), path)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let mut repl = REPL::new(">> ");
        repl.run_loop();
    } else if args.len() == 2 {
        let program = match read_program(&args[1]) {
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
    } else {
        print!("Invalid arguments");
        exit(1);
    }
}

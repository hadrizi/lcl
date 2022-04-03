mod lexer;
mod lib;
mod repl;
mod tests;

use lexer::tokenize;
use lexer::tokens::{Token, TokenType};
use lib::utils::{LocatedResult, Location};
use repl::REPL;
use std::env;
use std::fs::{self, File};
use std::io::{Error, Write};
use std::process::{exit, Command};

fn read_program(path: &str) -> LocatedResult<Vec<Token>> {
    let data = fs::read_to_string(path).expect("failed to read from file");
    tokenize(data.as_str(), path)
}

fn compile(program: Vec<Token>) -> Result<(), Error> {
    let mut output = File::create("output.asm").expect("failed to create asm file");
    let mut markers = Vec::<(String, Location)>::new();

    writeln!(output, "global _start")?;
    writeln!(output, "section .text")?;

    // Print function
    writeln!(output, "print:")?;
    writeln!(output, "\tsub     rsp, 40")?;
    writeln!(output, "\tmov     rsi, rdi")?;
    writeln!(output, "\tmov     r10, -3689348814741910323")?;
    writeln!(output, "\tmov     BYTE [rsp+19], 10")?;
    writeln!(output, "\tlea     rcx, [rsp+18]")?;
    writeln!(output, "\tlea     r8, [rsp+20]")?;
    writeln!(output, ".L2:")?;
    writeln!(output, "\tmov     rax, rsi")?;
    writeln!(output, "\tmov     r9, r8")?;
    writeln!(output, "\tmul     r10")?;
    writeln!(output, "\tmov     rax, rsi")?;
    writeln!(output, "\tsub     r9, rcx")?;
    writeln!(output, "\tshr     rdx, 3")?;
    writeln!(output, "\tlea     rdi, [rdx+rdx*4]")?;
    writeln!(output, "\tadd     rdi, rdi")?;
    writeln!(output, "\tsub     rax, rdi")?;
    writeln!(output, "\tadd     eax, 48")?;
    writeln!(output, "\tmov     BYTE [rcx], al")?;
    writeln!(output, "\tmov     rax, rsi")?;
    writeln!(output, "\tmov     rsi, rdx")?;
    writeln!(output, "\tmov     rdx, rcx")?;
    writeln!(output, "\tsub     rcx, 1")?;
    writeln!(output, "\tcmp     rax, 9")?;
    writeln!(output, "\tja      .L2")?;
    writeln!(output, "\tsub     rdx, r8")?;
    writeln!(output, "\tmov     edi, 1")?;
    writeln!(output, "\txor     eax, eax")?;
    writeln!(output, "\tlea     rsi, [rsp+20+rdx]")?;
    writeln!(output, "\tmov     rdx, r9")?;
    writeln!(output, "\tmov     rax, 1")?;
    writeln!(output, "\tsyscall")?;
    writeln!(output, "\tadd     rsp, 40")?;
    writeln!(output, "\tret")?;

    writeln!(output, "_start:")?;
    for op in program.iter() {
        match &op.ttype {
            TokenType::Integer(x) => {
                writeln!(output, "\t; Push({})", &x)?;
                writeln!(output, "\tmov  rax, {}", &x)?;
                writeln!(output, "\tpush rax")?;
            }
            TokenType::Plus => {
                writeln!(output, "\t; Plus")?;
                writeln!(output, "\tpop  rax")?;
                writeln!(output, "\tpop  rbx")?;
                writeln!(output, "\tadd  rax, rbx")?;
                writeln!(output, "\tpush rax")?;
            }
            TokenType::Minus => {
                writeln!(output, "\t; Minus")?;
                writeln!(output, "\tpop  rax")?;
                writeln!(output, "\tpop  rbx")?;
                writeln!(output, "\tadd  rbx, rax")?;
                writeln!(output, "\tpush rax")?;
            }
            TokenType::Dot => {
                writeln!(output, "\t; Dot")?;
                writeln!(output, "\tpop  rdi")?;
                writeln!(output, "\tcall print")?;
            }
            TokenType::Identifier(x) => writeln!(output, "\t; Identifier found {}", &x)?,
            TokenType::Less => {
                writeln!(output, "\t; Less")?;
                writeln!(output, "\tmov rcx, 0")?;
                writeln!(output, "\tmov rdx, 1")?;
                writeln!(output, "\tpop rbx")?;
                writeln!(output, "\tpop rax")?;
                writeln!(output, "\tcmp rax, rbx")?;
                writeln!(output, "\tcmovl rcx, rdx")?;
                writeln!(output, "\tpush rcx")?;
            }
            TokenType::Greater => {
                writeln!(output, "\t; Greater")?;
                writeln!(output, "\tmov rcx, 0")?;
                writeln!(output, "\tmov rdx, 1")?;
                writeln!(output, "\tpop rbx")?;
                writeln!(output, "\tpop rax")?;
                writeln!(output, "\tcmp rax, rbx")?;
                writeln!(output, "\tcmovg rcx, rdx")?;
                writeln!(output, "\tpush rcx")?;
            }
            TokenType::Equal => {
                writeln!(output, "\t; Equal")?;
                writeln!(output, "\tmov rcx, 0")?;
                writeln!(output, "\tmov rdx, 1")?;
                writeln!(output, "\tpop rax")?;
                writeln!(output, "\tpop rbx")?;
                writeln!(output, "\tcmp rax, rbx")?;
                writeln!(output, "\tcmove rcx, rdx")?;
                writeln!(output, "\tpush rcx")?;
            }
            TokenType::NotEqual => {
                writeln!(output, "\t; NotEqual")?;
                writeln!(output, "\tmov rcx, 0")?;
                writeln!(output, "\tmov rdx, 1")?;
                writeln!(output, "\tpop rax")?;
                writeln!(output, "\tpop rbx")?;
                writeln!(output, "\tcmp rax, rbx")?;
                writeln!(output, "\tcmovne rcx, rdx")?;
                writeln!(output, "\tpush rcx")?;
            }
            TokenType::If(marker) => {
                writeln!(output, "\t; If")?;
                writeln!(output, "\tpop rax")?;
                writeln!(output, "\ttest rax, rax")?;
                writeln!(output, "\tjz end_{}", &marker)?;
                markers.push((marker.to_string(), op.loc.clone()));
            }
            TokenType::End => {
                if !markers.is_empty() {
                    writeln!(output, "\t; End")?;
                    writeln!(output, "\tend_{}:", markers.pop().unwrap().0)?;
                }
            }
        }
    }

    if !markers.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!(
                "CompilationError: not enclosed block at {}",
                markers.last().unwrap().1
            ),
        ));
    }

    // exit syscall
    writeln!(output, "mov rax, 60")?;
    writeln!(output, "mov rdi, 0")?;
    writeln!(output, "syscall")?;
    writeln!(output, "ret")?;

    Command::new("nasm")
        .args(["-felf64", "output.asm"])
        .output()
        .expect("failed to run nasm");
    Command::new("ld")
        .args(["-o", "output", "output.o"])
        .output()
        .expect("failed to run ld");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let mut repl = REPL::new(">> ");
        repl.run_loop();
    } else if args.len() == 2 {
        match read_program(&args[1]) {
            Ok(program) => compile(program).expect("failed to compile a program"),
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    } else {
        print!("Invalid arguments");
        exit(1);
    }
}

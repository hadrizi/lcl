use std::{
    fs::File,
    io::{Error, Write},
    process::Command,
    str::from_utf8,
};

use crate::{
    lexer::tokens::{Token, TokenType},
    lib::utils::Location,
};

pub fn compile(program: Vec<Token>) -> Result<(), Error> {
    let mut output = File::create("output.asm").expect("failed to create asm file");
    let mut markers = Vec::<(usize, Location)>::new();
    let mem_capacity = 262144;

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
    for (idx, op) in program.iter().enumerate() {
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
                writeln!(output, "\tsub  rbx, rax")?;
                writeln!(output, "\tpush rbx")?;
            }
            TokenType::Dot => {
                writeln!(output, "\t; Dot")?;
                writeln!(output, "\tpop  rdi")?;
                writeln!(output, "\tcall print")?;
            }
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
            TokenType::If => {
                writeln!(output, "\t; If")?;
                writeln!(output, "\tpop rax")?;
                writeln!(output, "\ttest rax, rax")?;
                writeln!(output, "\tjz .e{}", &idx)?;
                markers.push((idx, op.loc.clone()));
            }
            TokenType::Else => {
                writeln!(output, "\tjmp .e{}", &idx)?;
                writeln!(output, ".e{}:", markers.pop().unwrap().0)?;
                markers.push((idx, op.loc.clone()));
            }
            TokenType::While => {
                writeln!(output, "\t; While:start of loop condition")?;
                writeln!(output, "l{}:", &idx)?;
                markers.push((idx, op.loc.clone()));
            }
            TokenType::Do => {
                if !markers.is_empty() {
                    let i = markers.last().unwrap().0;
                    writeln!(output, "\t; Do:end of loop condition")?;
                    writeln!(output, "\tpop rax")?;
                    writeln!(output, "\ttest rax, rax")?;
                    writeln!(output, "\tjz .e{}", &i)?;
                } else {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("CompilationError: unexpected do at {}", op.loc),
                    ));
                }
            }
            TokenType::End => {
                if !markers.is_empty() {
                    let i = markers.pop().unwrap().0;
                    match program[i].ttype {
                        TokenType::While => {
                            writeln!(output, "\tjmp l{}", i)?;
                            writeln!(output, ".e{}:", i)?;
                        }
                        _ => writeln!(output, ".e{}:", i)?,
                    };
                } else {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("CompilationError: unexpected end of block at {}", op.loc),
                    ));
                }
            }
            TokenType::Identifier(ident) => match ident.as_str() {
                "dup" => {
                    writeln!(output, "\t; DUP")?;
                    writeln!(output, "\tpop rax")?;
                    writeln!(output, "\tpush rax")?;
                    writeln!(output, "\tpush rax")?;
                }
                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("CompilationError: {} is not defined at {}", ident, op.loc),
                    ))
                }
            },
            TokenType::Mem => {
                writeln!(output, "\t; MEM")?;
                writeln!(output, "\tpush mem")?;
            }
            TokenType::Store => {
                writeln!(output, "\t; Store")?;
                writeln!(output, "\tpop rax")?;
                writeln!(output, "\tpop rbx")?;
                writeln!(output, "\tmov [rbx], rax")?;
            }
            TokenType::Load => {
                writeln!(output, "\t; Load")?;
                writeln!(output, "\tpop rax")?;
                writeln!(output, "\txor rbx, rbx")?;
                writeln!(output, "\tmov rbx, [rax]")?;
                writeln!(output, "\tpush rbx")?;
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
    writeln!(output, "\tmov rax, 60")?;
    writeln!(output, "\tmov rdi, 0")?;
    writeln!(output, "\tsyscall")?;
    writeln!(output, "\tret")?;

    writeln!(output, "section .bss")?;
    writeln!(output, "\tmem resq {}", mem_capacity)?;

    let output = Command::new("nasm")
        .args(["-felf64", "output.asm"])
        .output()
        .expect("failed to run nasm");
    if output.stderr.len() > 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("{}", from_utf8(&output.stderr).unwrap()),
        ));
    }

    let output = Command::new("ld")
        .args(["-o", "output", "output.o"])
        .output()
        .expect("failed to run ld");
    if output.stderr.len() > 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("{}", from_utf8(&output.stderr).unwrap()),
        ));
    }

    Ok(())
}

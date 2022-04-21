use std::{
    fs::File,
    io::{BufWriter, Error, Result, Write},
    process::Command,
    str::from_utf8,
};

use crate::{
    lexer::tokens::{Token, TokenType},
    lib::utils::Location,
};

struct Compiler {
    handler: BufWriter<File>,
    markers: Vec<(usize, Location)>,
    mem_capacity: i32,
}

impl Compiler {
    fn new(outfile: &str) -> Self {
        let file =
            File::create(format!("{}.{}", &outfile, "asm")).expect("failed to create asm file");
        let handler = BufWriter::new(file);
        Self {
            handler,
            mem_capacity: 262144,
            markers: Vec::new(),
        }
    }

    fn translate_tokens(&mut self, program: &Vec<Token>) -> Result<()> {
        self.headers()?;
        for (idx, token) in program.iter().enumerate() {
            let asm = self.to_asm(token, idx, program)?;
            writeln!(self.handler, "{}", asm)?;
        }
        self.footers()?;

        if !self.markers.is_empty() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "CompilationError: not enclosed block at {}",
                    self.markers.last().unwrap().1
                ),
            ));
        }

        self.flush()?;
        Ok(())
    }

    fn to_asm(&mut self, token: &Token, idx: usize, program: &Vec<Token>) -> Result<String> {
        match &token.ttype {
            TokenType::Integer(x) => {
                Ok(format!("\t; Push({0})\n\tmov  rax, {0}\n\tpush rax", x))
            }
            TokenType::Plus => {
                Ok(format!("\t; Plus\n\tpop  rax\n\tpop  rbx\n\tadd  rax, rbx\n\tpush rax"))
            }
            TokenType::Minus => {
                Ok(format!("\t; Minus\n\tpop  rax\n\tpop  rbx\n\tsub  rbx, rax\n\tpush rbx"))
            }
            TokenType::Dot => {
                Ok(format!("\t; Dot\n\tpop  rdi\n\tcall print"))
            }
            TokenType::Less => {
                Ok(format!("\t; Less\n\tmov rcx, 0\n\tmov rdx, 1\n\tpop rbx\n\tpop rax\n\tcmp rax, rbx\n\tcmovl rcx, rdx\n\tpush rcx"))
            }
            TokenType::Greater => {
                Ok(format!("\t; Greater\n\tmov rcx, 0\n\tmov rdx, 1\n\tpop rbx\n\tpop rax\n\tcmp rax, rbx\n\tcmovg rcx, rdx\n\tpush rcx"))
            }
            TokenType::Equal => {
                Ok(format!("\t; Equal\n\tmov rcx, 0\n\tmov rdx, 1\n\tpop rax\n\tpop rbx\n\tcmp rax, rbx\n\tcmove rcx, rdx\n\tpush rcx"))
            }
            TokenType::NotEqual => {
                Ok(format!("\t; NotEqual\n\tmov rcx, 0\n\tmov rdx, 1\n\tpop rax\n\tpop rbx\n\tcmp rax, rbx\n\tcmovne rcx, rdx\n\tpush rcx"))
            }
            TokenType::If => {
                self.markers.push((idx, token.loc.clone()));
                Ok(format!("\t; If\n\tpop rax\n\ttest rax, rax\n\tjz e{}", idx))
            }
            TokenType::Else => {
                if let Some((m, _)) = self.markers.pop() {
                    let r = format!("\tjmp e{}\ne{}:", idx, &m);
                    self.markers.push((idx, token.loc.clone()));
                    Ok(r)
                } else {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("CompilationError: unexpected `else` at {}", token.loc),
                    ));
                }
            }
            TokenType::While => {
                self.markers.push((idx, token.loc.clone()));
                Ok(format!("\t; While:start of loop condition\nl{}:", idx))
            }
            TokenType::Do => {
                if let Some((m, _)) = self.markers.last() {
                    Ok(format!("\t; Do:end of loop condition\n\tpop rax\n\ttest rax, rax\n\tjz e{}", m))
                } else {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("CompilationError: unexpected `do` at {}", token.loc),
                    ));
                }
            }
            TokenType::End => {
                if let Some((m, _)) = self.markers.pop() {
                    match program[m].ttype {
                        TokenType::While => {
                            Ok(format!("\tjmp l{0}\ne{0}:", &m))
                        }
                        _ => Ok(format!("e{}:", &m))
                    }
                } else {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("CompilationError: unexpected end of block at {}", token.loc),
                    ));
                }
            }
            TokenType::Identifier(ident) => match ident.as_str() {
                "dup" => {
                    Ok(format!("\t; DUP\n\tpop rax\n\tpush rax\n\tpush rax"))
                }
                "drop" => {
                    Ok(format!("\t; DROP\n\tpop rax\n\txor rax, rax"))
                }
                "swap" => {
                    Ok(format!("\t; SWAP\n\tpop rax\n\tpop rbx\n\tpush rax\n\tpush rbx"))
                }
                "over" => {
                    Ok(format!("\t; OVER\n\tpop rax\n\tpop rbx\n\tpush rbx\n\tpush rax\n\tpush rbx"))
                }
                "rot" => {
                    Ok(format!("\t; ROT\n\tpop rax\n\tpop rbx\n\tpop rcx\n\tpush rbx\n\tpush rax\n\tpush rcx"))
                }
                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("CompilationError: {} is not defined at {}", ident, token.loc),
                    ))
                }
            },
            TokenType::Mem => {
                Ok(format!("\t; MEM\n\tpush mem"))
            }
            TokenType::Store => {
                Ok(format!("\t; Store\n\tpop rax\n\tpop rbx\n\tmov [rbx], rax"))
            }
            TokenType::Load => {
                Ok(format!("\t; Load\n\tpop rax\n\txor rbx, rbx\n\tmov rbx, [rax]\n\tpush rbx"))
            }
        }
    }

    fn headers(&mut self) -> Result<()> {
        writeln!(self.handler, "global _start")?;
        writeln!(self.handler, "section .text")?;

        // Print function
        writeln!(self.handler, "print:")?;
        writeln!(self.handler, "\tsub     rsp, 40")?;
        writeln!(self.handler, "\tmov     rsi, rdi")?;
        writeln!(self.handler, "\tmov     r10, -3689348814741910323")?;
        writeln!(self.handler, "\tmov     BYTE [rsp+19], 10")?;
        writeln!(self.handler, "\tlea     rcx, [rsp+18]")?;
        writeln!(self.handler, "\tlea     r8, [rsp+20]")?;
        writeln!(self.handler, ".L2:")?;
        writeln!(self.handler, "\tmov     rax, rsi")?;
        writeln!(self.handler, "\tmov     r9, r8")?;
        writeln!(self.handler, "\tmul     r10")?;
        writeln!(self.handler, "\tmov     rax, rsi")?;
        writeln!(self.handler, "\tsub     r9, rcx")?;
        writeln!(self.handler, "\tshr     rdx, 3")?;
        writeln!(self.handler, "\tlea     rdi, [rdx+rdx*4]")?;
        writeln!(self.handler, "\tadd     rdi, rdi")?;
        writeln!(self.handler, "\tsub     rax, rdi")?;
        writeln!(self.handler, "\tadd     eax, 48")?;
        writeln!(self.handler, "\tmov     BYTE [rcx], al")?;
        writeln!(self.handler, "\tmov     rax, rsi")?;
        writeln!(self.handler, "\tmov     rsi, rdx")?;
        writeln!(self.handler, "\tmov     rdx, rcx")?;
        writeln!(self.handler, "\tsub     rcx, 1")?;
        writeln!(self.handler, "\tcmp     rax, 9")?;
        writeln!(self.handler, "\tja      .L2")?;
        writeln!(self.handler, "\tsub     rdx, r8")?;
        writeln!(self.handler, "\tmov     edi, 1")?;
        writeln!(self.handler, "\txor     eax, eax")?;
        writeln!(self.handler, "\tlea     rsi, [rsp+20+rdx]")?;
        writeln!(self.handler, "\tmov     rdx, r9")?;
        writeln!(self.handler, "\tmov     rax, 1")?;
        writeln!(self.handler, "\tsyscall")?;
        writeln!(self.handler, "\tadd     rsp, 40")?;
        writeln!(self.handler, "\tret")?;

        writeln!(self.handler, "_start:")?;

        Ok(())
    }

    fn footers(&mut self) -> Result<()> {
        writeln!(self.handler, "\tmov rax, 60")?;
        writeln!(self.handler, "\tmov rdi, 0")?;
        writeln!(self.handler, "\tsyscall")?;
        writeln!(self.handler, "\tret")?;

        writeln!(self.handler, "section .bss")?;
        writeln!(self.handler, "\tmem resq {}", self.mem_capacity)?;

        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        self.handler.flush()
    }
}

pub fn compile(program: &mut Vec<Token>, out: &str) -> Result<()> {
    let mut compiler = Compiler::new(out);
    compiler.translate_tokens(program)?;

    let output = Command::new("nasm")
        .args(["-felf64", format!("{}.{}", &out, "asm").as_str()])
        .output()
        .expect("failed to run nasm");
    if output.stderr.len() > 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("{}", from_utf8(&output.stderr).unwrap()),
        ));
    }

    let output = Command::new("ld")
        .args(["-o", out, format!("{}.{}", &out, "o").as_str()])
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

mod capture;

use std::{
    fs::File,
    io::{BufWriter, Error, Result, Write},
    process::Command,
    str::from_utf8, collections::HashMap,
};

use crate::{
    lexer::tokens::{TargetType, Token, TokenType},
    lib::utils::Location,
};

use self::capture::Capture;

struct Compiler {
    handler: BufWriter<File>,
    markers: Vec<(usize, Location)>,
    mem_capacity: i32,

    functions: HashMap<String, (String, usize, bool, bool)>,
    capture: Option<Capture>
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
            capture: None,
            functions: HashMap::new()
        }
    }

    fn translate_tokens(&mut self, program: &[Token]) -> Result<()> {
        let mut start_body = String::new();
        for (idx, token) in program.iter().enumerate() {
            let asm = self.token_to_asm(token, idx, program)?;
            if self.capture.is_some() {
                self.capture.as_mut().unwrap().push_asm(&asm)
            } else {
                start_body.push_str(&asm);
            }
        }

        if !self.markers.is_empty() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "CompilationError: not enclosed block at {}",
                    self.markers.last().unwrap().1
                ),
            ));
        }

        if let Some(c) = &self.capture {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "CompilationError: not enclosed function at {}",
                    c.loc
                ),
            ));
        }
        
        self.headers()?;
        writeln!(self.handler, "{}", &start_body)?;
        self.footers()?;
        self.flush()?;
        Ok(())
    }

    fn token_to_asm(&mut self, token: &Token, idx: usize, program: &[Token]) -> Result<String> {
        match &token.ttype {
            TokenType::Plus => {
                Ok("\t; Plus\n\tpop  rax\n\tpop  rbx\n\tadd  rax, rbx\n\tpush rax\n".to_string())
            }
            TokenType::Minus => {
                Ok("\t; Minus\n\tpop  rax\n\tpop  rbx\n\tsub  rbx, rax\n\tpush rbx\n".to_string())
            }
            TokenType::Dot => {
                Ok("\t; Dot\n\tpop  rdi\n\tcall print\n".to_string())
            }
            TokenType::Less => {
                Ok("\t; Less\n\tmov rcx, 0\n\tmov rdx, 1\n\tpop rbx\n\tpop rax\n\tcmp rax, rbx\n\tcmovl rcx, rdx\n\tpush rcx\n".to_string())
            }
            TokenType::Greater => {
                Ok("\t; Greater\n\tmov rcx, 0\n\tmov rdx, 1\n\tpop rbx\n\tpop rax\n\tcmp rax, rbx\n\tcmovg rcx, rdx\n\tpush rcx\n".to_string())
            }
            TokenType::Equal => {
                Ok("\t; Equal\n\tmov rcx, 0\n\tmov rdx, 1\n\tpop rax\n\tpop rbx\n\tcmp rax, rbx\n\tcmove rcx, rdx\n\tpush rcx\n".to_string())
            }
            TokenType::NotEqual => {
                Ok("\t; NotEqual\n\tmov rcx, 0\n\tmov rdx, 1\n\tpop rax\n\tpop rbx\n\tcmp rax, rbx\n\tcmovne rcx, rdx\n\tpush rcx\n".to_string())
            }
            TokenType::If => {
                self.markers.push((idx, token.loc.clone()));
                Ok(format!("\t; If\n\tpop rax\n\ttest rax, rax\n\tjz e{}\n", idx))
            }
            TokenType::Else => {
                if let Some((m, _)) = self.markers.pop() {
                    let r = format!("\tjmp e{}\ne{}:\n", idx, &m);
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
                Ok(format!("\t; While:start of loop condition\nl{}:\n", idx))
            }
            TokenType::Do => {
                if let Some((m, _)) = self.markers.last() {
                    Ok(format!("\t; Do:end of loop condition\n\tpop rax\n\ttest rax, rax\n\tjz e{}\n", m))
                } else if self.capture.is_some() && self.capture.as_ref().unwrap().initializing {
                    self.capture.as_mut().unwrap().initializing = false;
                    Ok("".to_string())
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
                            Ok(format!("\tjmp l{0}\ne{0}:\n", &m))
                        }
                        _ => Ok(format!("e{}:\n", &m))
                    }
                } else if self.capture.is_some() {
                    self.functions.insert(
                        self.capture.as_ref().unwrap().get_name().to_string(), 
                        (
                                self.capture.as_mut().unwrap().get_source().to_string(), 
                                self.capture.as_mut().unwrap().last_offset(),
                                self.capture.as_mut().unwrap().returning,
                                self.capture.as_mut().unwrap().inline
                            )
                    );
                    self.capture = None;
                    Ok("".to_string())
                } else {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("CompilationError: unexpected end of block at {}", token.loc),
                    ));
                }
            }
            TokenType::Identifier(ident) => match ident.as_str() {
                "dup" => {
                    Ok("\t; DUP\n\tpop rax\n\tpush rax\n\tpush rax\n".to_string())
                }
                "drop" => {
                    Ok("\t; DROP\n\tpop rax\n\txor rax, rax\n".to_string())
                }
                "swap" => {
                    Ok("\t; SWAP\n\tpop rax\n\tpop rbx\n\tpush rax\n\tpush rbx\n".to_string())
                }
                "over" => {
                    Ok("\t; OVER\n\tpop rax\n\tpop rbx\n\tpush rbx\n\tpush rax\n\tpush rbx\n".to_string())
                }
                "rot" => {
                    Ok("\t; ROT\n\tpop rax\n\tpop rbx\n\tpop rcx\n\tpush rbx\n\tpush rax\n\tpush rcx\n".to_string())
                }
                name if self.capture.is_some() && !self.capture.as_ref().unwrap().has_name() => {
                    self.capture.as_mut().unwrap().set_name(name);
                    Ok("".to_string())
                }
                name if self.capture.is_some() && self.capture.as_ref().unwrap().initializing && self.capture.as_ref().unwrap().has_name() => {
                    self.capture.as_mut().unwrap().add_local_var(name);
                    Ok("".to_string())
                }
                name
                    if 
                        self.capture.is_some() &&
                        !self.capture.as_ref().unwrap().initializing &&
                        self.capture.as_ref().unwrap().has_local_var(name) => 
                {
                    let var = self.capture.as_ref().unwrap().get_local_var(name);
                    Ok(format!("\t; Push {}\n\tmov rax, {}\n\tpush rax\n", name, var))
                }
                other => {
                    if let Some((source, size, returning, inline)) = self.functions.get(other) {
                        if !inline {
                            Ok(format!(
                                "\t; Call {0}\n\tcall {0}\n\tadd rsp, {1}\n{2}", 
                                other, 
                                size, 
                                if *returning { "\tpush rax\n" } else { "" }
                            ))
                        } else {
                            Ok(format!("\t; Inline call {}\n{}", &other, source))
                        }
                    } else {
                        return Err(Error::new(
                            std::io::ErrorKind::Other,
                            format!("CompilationError: {} is not defined at {}", ident, token.loc),
                        ))
                    }
                }
            }
            TokenType::Mem => {
                Ok("\t; MEM\n\tpush mem".to_string())
            }
            TokenType::Push(target) => {
                match target {
                    TargetType::Integer(n) => Ok(format!("\t; Push {0}\n\tmov  rax, {0}\n\tpush rax\n", n)),
                    TargetType::Memory => Ok("\t; Load\n\tpop rax\n\txor rbx, rbx\n\tmov rbx, [rax]\n\tpush rbx\n".to_string()),
                    TargetType::Regsiter(i) => {
                        let reg = self.get_register(*i)?;
                        Ok(format!("\t; Push {0}\n\tpush {0}\n", reg))
                    },
                }
            }
            TokenType::Pop(target) => {
                match target {
                    TargetType::Memory => Ok("\t; Store\n\tpop rax\n\tpop rbx\n\tmov [rbx], rax\n".to_string()),
                    TargetType::Regsiter(i) => {
                        let reg = self.get_register(*i)?;
                        Ok(format!("\t; Pop {0}\n\tpop {0}\n", reg))
                    },
                    TargetType::Integer(_) => Err(Error::new(
                        std::io::ErrorKind::Other, 
                        format!("CompilationError: cannot pop from immediate integer value at {}", token.loc)
                    )),
                }
            }
            TokenType::Function => {
                if self.capture.is_none() {
                    self.capture = Some(Capture::new(token.loc.clone(), false));
                }
                Ok("".to_string())
            }
            TokenType::Inline => {
                if self.capture.is_none() {
                    self.capture = Some(Capture::new(token.loc.clone(), true));
                }
                Ok("".to_string())
            }
            TokenType::Multiply => unimplemented!(),
            TokenType::Divide => unimplemented!(),
            TokenType::Mod => unimplemented!(),
        }
    }

    fn get_register(&self, idx: usize) -> Result<String> {
        match &idx {
            1 => Ok("rax".to_string()),
            2 => Ok("rbx".to_string()),
            3 => Ok("rcx".to_string()),
            4 => Ok("rdx".to_string()),
            _ => Err(Error::new(
                std::io::ErrorKind::Other, 
                format!("CompilationError: invalid register index {}", idx)
            ))
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

        for func in self.functions.iter() {
            if !func.1.3 {
                writeln!(self.handler, "{}", func.1.0)?;
            }
        }

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
    if !output.stderr.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            from_utf8(&output.stderr).unwrap().to_string(),
        ));
    }

    let output = Command::new("ld")
        .args(["-o", out, format!("{}.{}", &out, "o").as_str()])
        .output()
        .expect("failed to run ld");
    if !output.stderr.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            from_utf8(&output.stderr).unwrap().to_string(),
        ));
    }

    Ok(())
}

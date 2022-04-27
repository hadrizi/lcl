use std::io::{
    stderr, stdin, stdout, BufRead, BufReader, BufWriter, Error, ErrorKind, Read, Stderr, Stdin,
    Stdout, Write,
};

use crate::{
    lexer::{
        tokenize,
        tokens::{Token, TokenType},
    },
    lib::{
        constants::{PKG_DESCRIPTION, PKG_NAME, VERSION},
        utils::LocatedResult,
    },
};

struct Stack(Vec<i64>);

impl Stack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn pop(&mut self) -> Result<i64, Error> {
        self.0
            .pop()
            .ok_or_else(|| Error::new(ErrorKind::Other, "StackError: stack is empty"))
    }

    fn push(&mut self, v: i64) {
        self.0.push(v);
    }
}

pub struct Repl {
    stack: Stack,
    prompt: String,
    memory: [i64; 262144],
    input_handle: BufReader<Stdin>,
    output_handle: BufWriter<Stdout>,
    error_handle: BufWriter<Stderr>,
}

impl Repl {
    pub fn new(prompt: &str) -> Self {
        let stdin = stdin();
        let stdout = stdout();
        let stderr = stderr();
        Repl {
            stack: Stack::new(),
            prompt: String::from(prompt),
            input_handle: BufReader::new(stdin),
            output_handle: BufWriter::new(stdout),
            error_handle: BufWriter::new(stderr),
            memory: [0; 262144],
        }
    }

    fn read(&mut self) -> LocatedResult<Vec<Token>> {
        print!("{}", self.prompt);
        stdout().flush().expect("failed to flush stdout");
        let input_result = self
            .input_handle
            .by_ref()
            .lines()
            .next()
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "failed to read input stream"))
            .and_then(|inner| inner);

        match input_result {
            Ok(src) => {
                let src = src.as_str();
                tokenize(src, "<stdin>")
            }
            Err(e) => {
                writeln!(self.error_handle.by_ref(), "{}", e).unwrap();
                Ok(vec![])
            }
        }
    }

    fn eval(&mut self, op: Token) -> Result<(), Error> {
        match op.ttype {
            TokenType::Plus => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a + b);
            }
            TokenType::Minus => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(b - a);
            }
            TokenType::Dot => writeln!(self.output_handle, "{}", self.stack.pop()?)?,
            TokenType::Less => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push((b < a) as i64);
            }
            TokenType::Greater => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push((b > a) as i64);
            }
            TokenType::Equal => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push((a == b) as i64);
            }
            TokenType::NotEqual => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push((a != b) as i64);
            }
            TokenType::Identifier(_) => {
                writeln!(
                    self.error_handle,
                    "identifiers are not supported in the interactive shell",
                )?;
            }
            TokenType::If => writeln!(
                self.error_handle,
                "control flow is not supported in the interactive shell",
            )?,
            TokenType::Else => writeln!(
                self.error_handle,
                "control flow is not supported in the interactive shell",
            )?,
            TokenType::While => writeln!(
                self.error_handle,
                "loops are not supported in the interactive shell",
            )?,
            TokenType::Do => writeln!(
                self.error_handle,
                "loops are not supported in the interactive shell",
            )?,
            TokenType::End => writeln!(
                self.error_handle,
                "control flow is not supported in the interactive shell",
            )?,
            TokenType::Mem => self.stack.push(0),
            TokenType::Push(target) => match target {
                crate::lexer::tokens::TargetType::Integer(v) => self.stack.push(v),
                crate::lexer::tokens::TargetType::Regsiter(_) => writeln!(
                    self.error_handle,
                    "registers are not available in the interactive shell",
                )?,
                crate::lexer::tokens::TargetType::Memory => {
                    let a = self.stack.pop()?;
                    let b = self.memory[a as usize];
                    self.stack.push(b);
                }
            },
            TokenType::Pop(target) => match target {
                crate::lexer::tokens::TargetType::Integer(_) => writeln!(
                    self.error_handle,
                    "pop to immediate integer value is not allowed",
                )?,
                crate::lexer::tokens::TargetType::Regsiter(_) => writeln!(
                    self.error_handle,
                    "registers are not available in the interactive shell",
                )?,
                crate::lexer::tokens::TargetType::Memory => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.memory[b as usize] = a;
                }
            },
            TokenType::Multiply => unimplemented!(),
            TokenType::Divide => unimplemented!(),
            TokenType::Mod => unimplemented!(),
        }
        Ok(())
    }

    pub fn run_loop(&mut self) {
        println!(
            "{} {} interactive shell\n{}",
            PKG_NAME, VERSION, PKG_DESCRIPTION
        );
        let mut is_ok = true;
        loop {
            match self.read() {
                Ok(ops) => {
                    for op in ops {
                        if let Err(e) = self.eval(op) {
                            writeln!(self.error_handle, "{}", &e).unwrap();
                            is_ok = false;
                        };
                    }
                    if is_ok {
                        writeln!(self.output_handle, "ok").unwrap();
                    }
                }
                Err(e) => writeln!(self.error_handle, "{}", &e).unwrap(),
            }

            self.output_handle.flush().unwrap();
            self.error_handle.flush().unwrap();
        }
    }
}

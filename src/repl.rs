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
        constants::{PKG_NAME, VERSION},
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

pub struct REPL {
    stack: Stack,
    prompt: String,
    input_handle: BufReader<Stdin>,
    output_handle: BufWriter<Stdout>,
    error_handle: BufWriter<Stderr>,
}

impl REPL {
    pub fn new(prompt: &str) -> Self {
        let stdin = stdin();
        let stdout = stdout();
        let stderr = stderr();
        REPL {
            stack: Stack::new(),
            prompt: String::from(prompt),
            input_handle: BufReader::new(stdin),
            output_handle: BufWriter::new(stdout),
            error_handle: BufWriter::new(stderr),
        }
    }

    pub fn read(&mut self) -> LocatedResult<Vec<Token>> {
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

    pub fn eval(&mut self, op: Token) -> Result<(), Error> {
        match op.ttype {
            TokenType::Integer(x) => {
                self.stack.push(x);
            }
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
            TokenType::Identifier(_) => {
                writeln!(self.error_handle, "identifiers are not implemented",)?;
            }
        }
        Ok(())
    }

    pub fn run_loop(&mut self) {
        println!("{} {}", PKG_NAME, VERSION);
        loop {
            match self.read() {
                Ok(ops) => {
                    for op in ops {
                        match self.eval(op) {
                            Err(e) => writeln!(self.error_handle, "{}", &e).unwrap(),
                            _ => writeln!(self.output_handle, "ok").unwrap(),
                        };
                    }
                }
                Err(e) => writeln!(self.error_handle, "{}", &e).unwrap(),
            }

            self.output_handle.flush().unwrap();
            self.error_handle.flush().unwrap();
        }
    }
}

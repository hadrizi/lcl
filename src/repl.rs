use std::io::{stdin, stdout, BufRead, BufReader, Error, ErrorKind, Write};

use crate::{
    constants::{PKG_NAME, VERSION},
    lexer::{
        tokenize,
        tokens::{Token, TokenType},
        utils::LocatedResult,
    },
};

pub struct REPL {
    stack: Vec<i64>,
    prompt: String,
}

impl REPL {
    pub fn new(prompt: &str) -> Self {
        REPL {
            stack: Vec::new(),
            prompt: String::from(prompt),
        }
    }

    pub fn read(&self) -> LocatedResult<Vec<Token>> {
        print!("{}", self.prompt);
        stdout().flush().expect("failed to flush stdout");
        let result = BufReader::new(stdin())
            .lines()
            .next()
            .ok_or_else(|| Error::new(ErrorKind::Other, "failed to read stdin"))
            .and_then(|inner| inner)
            .expect("failed to read");
        if !result.is_empty() {
            let src = result.as_str();
            tokenize(src, "<stdin>")
        } else {
            Ok(vec![])
        }
    }

    pub fn eval(&mut self, op: Token) -> String {
        let mut result = String::new();
        match op.ttype {
            TokenType::Integer(x) => self.stack.push(x),
            TokenType::Plus => {
                let a = self.stack.pop().expect("stack is empty");
                let b = self.stack.pop().expect("stack is empty");
                self.stack.push(a + b);
            }
            TokenType::Minus => {
                let a = self.stack.pop().expect("stack is empty");
                let b = self.stack.pop().expect("stack is empty");
                self.stack.push(b - a);
            }
            TokenType::Dot => {
                result.push_str(format!("{}\n", self.stack.pop().expect("stack is empty")).as_str())
            }
            TokenType::Identifier(_) => todo!(),
        }
        result
    }

    pub fn run_loop(&mut self) {
        println!("{} {}", PKG_NAME, VERSION);
        loop {
            match self.read() {
                Ok(ops) => {
                    for op in ops {
                        let mut result = self.eval(op);
                        result.push_str("ok\n");
                        print!("{}", result);
                    }
                }
                Err(e) => eprintln!("{}", &e),
            }
        }
    }
}

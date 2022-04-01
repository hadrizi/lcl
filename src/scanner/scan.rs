use std::process;

use crate::scanner::file::File;
use crate::scanner::defs::{Token, TokenName};

pub struct Scanner {
    source_file: File,
    putback: Option<char>
}

impl Scanner {
    pub fn new(filename: String) -> Scanner {
        Scanner {
            source_file: File::new(filename),
            putback: None
        }
    }

    pub fn next(&mut self) -> char {
        let c: char;

        if self.putback.is_some() {
            c = self.putback.unwrap();
            self.putback = None;
            return c;
        }

        c = self.source_file.getc();
        c
    }

    pub fn skip(&mut self) -> char {
        let mut c = self.next();
        while c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\x0C' {
            c = self.next();
        }
        c
    }

    pub fn scan(&mut self, token: &mut Token) -> bool {
        let c = self.skip();
        match c {
            '+' => {token.token_name = TokenName::PLUS; true},
            '-' => {token.token_name = TokenName::MINUS; true},
            '*' => {token.token_name = TokenName::STAR; true},
            '/' => {token.token_name = TokenName::SLASH; true},
            '\0' => false,
            _ => {
                if c.is_digit(10) {
                    token.int_value = self.scanint(c);
                    token.token_name = TokenName::INTLIT;
                    true
                } else {
                    eprintln!(
                        "Unrecognized character {} on pos {} line {}\n", 
                        c, self.source_file.verbose_carry(), self.source_file.verbose_line()
                    );
                    process::exit(1);
                }
            }
            
        }
    }

    fn scanint(&mut self, c: char) -> i32 {
        let mut value = 0;
        let mut c_ = c;
        while c_.is_digit(10) {
            value = value * 10 + c_.to_digit(10).unwrap() as i32;
            c_ = self.next();
        }

        self.putback = Some(c_);
        value
    }
}


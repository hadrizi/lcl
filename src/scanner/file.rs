use std::{fs, };

#[derive(Debug)]
pub struct File {
    source:     String,
    carry:      usize,
    line:       i32
}

impl File {
    pub fn new(filename: String) -> File {
        File{
            source:     fs::read_to_string(filename).expect("Something went wrong reading the file"),
            carry:      0,
            line:       0
        }
    }

    pub fn getc(&mut self) -> char {
        let source_arr = self.source.as_bytes();
        if source_arr.len() <= self.carry {
            return '\0';
        }
        let c = source_arr[self.carry] as char;
        if c == '\n' {
            self.line += 1;
        }
        self.carry += 1;
        c
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn carry(&self) -> usize {
        self.carry
    }

    pub fn verbose_line(&self) -> i32 {
        self.line + 1
    }

    pub fn verbose_carry(&self) -> i32 {
        self.carry as i32
    }
}
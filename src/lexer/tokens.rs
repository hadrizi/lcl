use std::str::FromStr;

use crate::lib::{
    errors::LexingError,
    utils::{LexingResult, Location},
};

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub loc: Location,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TargetType {
    Integer(i64),
    Regsiter(usize),
    Memory,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Integer(i64),
    Identifier(String),
    Push(TargetType),
    Pop(TargetType),
    Function,
    Dot,
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,
    Less,
    Greater,
    Equal,
    NotEqual,
    Mem,
    If,
    Else,
    While,
    Do,
    End,
}

impl FromStr for TokenType {
    type Err = LexingError;

    fn from_str(s: &str) -> LexingResult<Self> {
        match s {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            "%" => Ok(Self::Mod),
            "." => Ok(Self::Dot),
            "<" => Ok(Self::Less),
            ">" => Ok(Self::Greater),
            "=" => Ok(Self::Equal),
            "!=" => Ok(Self::NotEqual),
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),
            "end" => Ok(Self::End),
            "while" => Ok(Self::While),
            "do" => Ok(Self::Do),
            "mem" => Ok(Self::Mem),
            "fn" => Ok(Self::Function),
            other if other.starts_with('!') && other.len() == 1 => {
                Ok(Self::Push(TargetType::Memory))
            }
            other if other.starts_with('@') && other.len() == 1 => {
                Ok(Self::Pop(TargetType::Memory))
            }
            other
                if other.starts_with('!')
                    && other.len() > 1
                    && other[1..].parse::<i64>().is_ok() =>
            {
                Ok(Self::Push(tokenize_number(&other[1..])?))
            }
            other if other.starts_with('!') && other.len() > 1 && other[1..].starts_with('r') => {
                Ok(Self::Push(tokenize_register(&other[2..])?))
            }
            other if other.starts_with('@') && other.len() > 1 && other[1..].starts_with('r') => {
                Ok(Self::Pop(tokenize_register(&other[2..])?))
            }
            // "!" => Ok(Self::Push(TargetType::Memory)),
            // "@" => Ok(Self::Pop(TargetType::Memory)),
            _ if s.parse::<i64>().is_ok() => Ok(Self::Push(tokenize_number(s)?)),
            _ if s.starts_with('_') || s.chars().next().unwrap().is_alphabetic() => {
                tokenize_identifier(s)
            }
            _ => Err(LexingError::Syntax(s.to_string())),
        }
    }
}

fn tokenize_number(number: &str) -> LexingResult<TargetType> {
    if let Ok(n) = number.parse::<i64>() {
        Ok(TargetType::Integer(n))
    } else {
        Err(LexingError::ParsingNumber(number.to_string()))
    }
}

fn tokenize_register(number: &str) -> LexingResult<TargetType> {
    if let Ok(n) = number.parse::<usize>() {
        Ok(TargetType::Regsiter(n))
    } else {
        Err(LexingError::RegisterIndex(number.to_string()))
    }
}

fn tokenize_identifier(ident: &str) -> LexingResult<TokenType> {
    let first_char = ident.chars().next().unwrap();
    if first_char.is_numeric() || first_char == '.' {
        return Err(LexingError::ParsingIdentifier(ident.to_string()));
    }

    Ok(TokenType::Identifier(ident.to_string()))
}

pub(super) fn tokenize_word(word: &str) -> LexingResult<(TokenType, usize)> {
    let ttype = TokenType::from_str(word)?;
    Ok((ttype, word.len()))
}

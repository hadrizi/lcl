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
pub enum TokenType {
    Integer(i64),
    Identifier(String),
    Dot,
    Plus,
    Minus,
    Less,
    Greater,
    Equal,
    NotEqual,
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
            _ if s.parse::<i64>().is_ok() => tokenize_number(s),
            _ if s.starts_with("_") || s.chars().next().unwrap().is_alphabetic() => {
                tokenize_identifier(s)
            }
            _ => Err(LexingError::Syntax(s.to_string())),
        }
    }
}

fn tokenize_number(number: &str) -> LexingResult<TokenType> {
    if let Ok(n) = number.parse::<i64>() {
        Ok(TokenType::Integer(n))
    } else {
        Err(LexingError::ParsingNumber(number.to_string()))
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

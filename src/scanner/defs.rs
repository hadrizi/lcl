#[derive(PartialEq)]
pub enum TokenName {
    PLUS,
    MINUS,
    STAR,
    SLASH,
    INTLIT,

    NOP
}

pub struct Token {
    pub token_name: TokenName,
    pub int_value: i32
}

impl Token {
    pub fn new() -> Token {
        Token{
            token_name: TokenName::NOP,
            int_value: 0
        }
    }
}

impl TokenName {
    pub fn value(&self) -> &str {
        match *self {
            TokenName::PLUS => "+",
            TokenName::MINUS => "-",
            TokenName::STAR => "*",
            TokenName::SLASH => "/",
            TokenName::INTLIT => "intlit",
            TokenName::NOP => "nop",
        }
    }
}
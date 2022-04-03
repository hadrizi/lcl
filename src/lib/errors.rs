use core::fmt;
use std::error::Error;

use super::utils::Location;

#[derive(Debug)]
pub enum LexingError {
    ParsingNumber(String),
    ParsingIdentifier(String),
    Syntax(String),
    UnexpectedEOF,
    NoMatch,
}

#[derive(Debug)]
pub struct LocatedError {
    pub loc: Location,
    pub error: LexingError,
}

impl LocatedError {
    pub fn new(loc: Location, error: LexingError) -> Self {
        Self { loc, error }
    }
}

impl fmt::Display for LocatedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {}", self.error, self.loc)
    }
}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::ParsingNumber(ref s) => {
                write!(f, "\t{}\n\t^\nParsingNumberError: not a number", s)
            }
            Self::ParsingIdentifier(ref s) => write!(
                f,
                "\t{}\n\t^\nParsingIdentifierError: invalid identifier",
                s
            ),
            Self::Syntax(ref s) => write!(f, "\t{}\n\t^\nSyntaxError: invalid syntax", s),
            Self::UnexpectedEOF => write!(f, "UnexpectedEOFError: unexpected end of file"),
            Self::NoMatch => write!(f, "NoMatchError: found no match"),
        }
    }
}

impl Error for LexingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            _ => None,
        }
    }
}

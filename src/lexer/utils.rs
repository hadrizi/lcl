use core::fmt;

use super::errors::{LexingError, LocatedError};

pub type LexingResult<T> = std::result::Result<T, LexingError>;
pub type LocatedResult<T> = std::result::Result<T, LocatedError>;

#[derive(Clone)]
pub struct Location {
    pub col: usize,
    pub row: usize,
    pub idx: usize,

    pub file: String,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            col: 1,
            row: 1,
            idx: 0,
            file: "".to_string(),
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.row, self.col)
    }
}

pub fn fetch_while<F>(data: &str, mut condition: F) -> LexingResult<(&str, usize)>
where
    F: FnMut(char) -> bool,
{
    let mut idx = 0;

    for ch in data.chars() {
        if !condition(ch) {
            break;
        }
        idx += ch.len_utf8();
    }

    if idx == 0 {
        Err(LexingError::NoMatch)
    } else {
        Ok((&data[..idx], idx))
    }
}

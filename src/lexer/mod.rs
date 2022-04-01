mod errors;
pub mod tokens;
pub mod utils;

use self::{
    errors::{LexingError, LocatedError},
    tokens::{tokenize_word, Token},
    utils::{fetch_while, LocatedResult, Location},
};

struct Lexer<'a> {
    loc: Location,
    src: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str, file: &'a str) -> Self {
        Self {
            src,
            loc: Location {
                file: file.to_string(),
                ..Default::default()
            },
        }
    }

    pub fn next_token(&mut self) -> LocatedResult<Option<Token>> {
        self.skip();

        if self.src.is_empty() {
            Ok(None)
        } else {
            let word = match fetch_while(self.src, |c| !c.is_whitespace()) {
                Ok((w, _)) => w,
                Err(_) => {
                    return Err(LocatedError::new(
                        self.loc.clone(),
                        LexingError::UnexpectedEOF,
                    ))
                }
            };

            let (ttype, size) = match tokenize_word(word) {
                Ok(r) => r,
                Err(e) => return Err(LocatedError::new(self.loc.clone(), e)),
            };

            let result = Some(Token {
                ttype,
                loc: self.loc.clone(),
            });
            self.loc.idx += size;
            self.loc.col += size;
            self.src = &self.src[size..];

            Ok(result)
        }
    }

    fn skip(&mut self) {
        let (whitespaces, newlines) = skip_whitespace(self.src);
        let comments = skip_comments(self.src);
        self.jump(whitespaces + comments, newlines);
    }

    fn jump(&mut self, len: usize, newlines: usize) {
        self.src = &self.src[len..];
        self.loc.idx += len;
        if newlines == 0 {
            self.loc.col += len;
        } else {
            self.loc.col = 1;
            self.loc.row += newlines;
        }
    }
}

fn skip_whitespace(data: &str) -> (usize, usize) {
    let mut newlines: usize = 0;
    match fetch_while(data, |ch| {
        if ch == '\n' {
            newlines += 1;
        };
        ch.is_whitespace()
    }) {
        Ok((_, skipped)) => (skipped, newlines),
        _ => (0, 0),
    }
}

fn skip_comments(data: &str) -> usize {
    let comment_patterns = [("//", "\n"), ("/*", "*/")];

    for pattern in comment_patterns.iter() {
        if data.starts_with(pattern.0) {
            let mut cut_data = data;
            while !cut_data.is_empty() && !cut_data.starts_with(pattern.1) {
                cut_data = &cut_data[data.chars().next().expect("String is empty").len_utf8()..];
            }
            if cut_data.chars().nth(0).unwrap() != '\n' {
                cut_data = &cut_data[pattern.1.len()..];
            }

            return data.len() - cut_data.len();
        }
    }
    0
}

pub fn tokenize(src: &str, file: &str) -> LocatedResult<Vec<Token>> {
    let mut lexer = Lexer::new(src, file);
    let mut program: Vec<Token> = Vec::new();

    while let Some(t) = lexer.next_token()? {
        program.push(t);
    }

    Ok(program)
}

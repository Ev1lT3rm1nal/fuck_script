use std::error::Error;
use std::fmt;

use crate::lexer::Position;
use crate::utils;

#[derive(Debug)]
pub struct DataError<'a> {
    pub text: &'a str,
    pub pos_start: Position<'a>,
    pub pos_end: Position<'a>,
    pub filename: &'a str,
}

#[derive(Debug)]
pub enum LexerError<'a> {
    IllegalCharError { char: char, data: DataError<'a> },
}

impl<'a> LexerError<'a> {}

impl<'a> Error for LexerError<'a> {}

impl<'a> fmt::Display for LexerError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::IllegalCharError { data, char } => {
                write!(
                    f,
                    "IllegalCharError: char '{}' in File \"{}:{}:{}\"\n{}",
                    char,
                    data.filename,
                    data.pos_start.ln + 1,
                    data.pos_start.col + 1,
                    utils::string_with_arrows(data.text, &data.pos_start, &data.pos_end)
                )
            }
        }
    }
}

use std::error::Error;
use std::fmt;

use crate::lexer::Position;
use crate::utils;

#[derive(Debug, Clone)]
pub struct DataError {
    pub text: String,
    pub pos_start: Position,
    pub pos_end: Position,
    pub filename: String,
}

#[derive(Debug, Clone)]
pub enum LexerError {
    IllegalCharError { char: char, data: DataError },
    InvalidSyntaxError { data: DataError },
}

impl LexerError {}

impl Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::IllegalCharError { data, char } => {
                write!(
                    f,
                    "IllegalCharError: char '{}' in File {}:{}:{}\n\n{}",
                    char,
                    data.filename,
                    data.pos_start.ln + 1,
                    data.pos_start.col + 1,
                    utils::string_with_arrows(data.text.clone(), &data.pos_start, &data.pos_end)
                )
            }
            LexerError::InvalidSyntaxError { data } => {
                write!(
                    f,
                    "InvalidSyntaxError in File {}:{}:{}\n\n{}",
                    data.filename,
                    data.pos_start.ln + 1,
                    data.pos_start.col + 1,
                    utils::string_with_arrows(data.text.clone(), &data.pos_start, &data.pos_end)
                )
            }
        }
    }
}

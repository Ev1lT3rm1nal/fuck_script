#![allow(dead_code)]

use crate::errors::{LexerError, DataError};

pub const DIGITS: &str = "0123456789";

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    STRING(String),
    CHAR(char),
    FLOAT(f64),
    INT(isize),
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EQUAL,
    ASSIGN,
    FUNCTION,
    LPARENT,
    RPARENT,
    LBRACKET,
    RBRACKET,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: TokenValue,
}

#[derive(Debug, Clone, Copy)]
pub struct Position<'a> {
    pub index: isize,
    pub ln: usize,
    pub col: isize,
    pub filename: &'a str,
    pub text: &'a str,
}

impl<'a> Position<'a> {
    pub fn new(filename: &'a str, text: &'a str) -> Position<'a> {
        Position {
            index: -1,
            ln: 0,
            col: -1,
            filename: filename,
            text: text,
        }
    }

    pub fn advance(&mut self, current_char: Option<char>) {
        self.index += 1;
        self.col += 1;

        if current_char.is_some() && current_char.unwrap() == '\n' {
            self.ln += 1;
            self.col = 0;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lexer<'a> {
    pub filename: &'a str,
    pub text: &'a str,
    pub pos: Position<'a>,
    pub current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(filename: &'a str, text: &'a str) -> Lexer<'a> {
        Lexer {
            filename: filename,
            text: text,
            pos: Position::new(filename, text),
            current_char: None,
        }
    }

    fn advance(&mut self) {
        self.pos.advance(self.current_char);
        self.current_char = if self.pos.index >= self.text.len() as isize {
            None
        } else {
            Some(self.text.chars().nth(self.pos.index as usize).unwrap())
        };
    }

    fn make_number(&mut self) -> Token {
        let mut str = String::new();
        let mut dot_count: usize = 0;

        for (i, c) in self.text.char_indices().skip(self.pos.index as usize) {
            if !DIGITS.contains(&c.to_string()) && c != '.' {
                break;
            }
            if c == '.' {
                if dot_count == 1 {
                    break;
                }
                dot_count += 1;
            }

            str.push(c);
            self.pos.index = i as isize;
            self.advance();
        }

        if dot_count == 0 {
            return Token {
                value: TokenValue::INT(str.parse::<isize>().unwrap()),
            };
        } else {
            return Token {
                value: TokenValue::FLOAT(str.parse::<f64>().unwrap()),
            };
        }
    }

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        self.advance();

        while self.current_char.is_some() {
            match self.current_char.unwrap() {
                '\t' | ' ' | '\n' | '\r' => self.advance(),
                '+' => {
                    tokens.push(Token {
                        value: TokenValue::PLUS,
                    });
                    self.advance();
                }
                '-' => {
                    tokens.push(Token {
                        value: TokenValue::MINUS,
                    });
                    self.advance();
                }
                '*' => {
                    tokens.push(Token {
                        value: TokenValue::MULTIPLY,
                    });
                    self.advance();
                }
                '/' => {
                    tokens.push(Token {
                        value: TokenValue::DIVIDE,
                    });
                    self.advance();
                }
                '(' => {
                    tokens.push(Token {
                        value: TokenValue::LPARENT,
                    });
                    self.advance();
                }
                ')' => {
                    tokens.push(Token {
                        value: TokenValue::RPARENT,
                    });
                    self.advance();
                }
                '{' => {
                    tokens.push(Token {
                        value: TokenValue::LBRACKET,
                    });
                    self.advance();
                }
                '}' => {
                    tokens.push(Token {
                        value: TokenValue::RBRACKET,
                    });
                    self.advance();
                }
                _ => {
                    if DIGITS.contains(&self.current_char.unwrap().to_string()) {
                        tokens.push(self.make_number());
                    } else {
                        let pos_start = self.pos.clone();
                        let char = self.current_char.unwrap();
                        self.advance();
                        return Err({

                            let filename = self.filename;
                            LexerError::IllegalCharError {
                                char: char,
                                data: DataError {
                                    text: self.text,
                                    filename,
                                    pos_start,
                                    pos_end: self.pos,
                                },
                            }
                        });
                    }
                }
            }
        }
        return Ok(tokens);
    }
}

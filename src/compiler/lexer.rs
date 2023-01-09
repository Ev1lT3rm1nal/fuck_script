use crate::errors::{DataError, LexerError};

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
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl Token {
    pub fn new(value: TokenValue, pos_start: Option<Position>, pos_end: Option<Position>) -> Token {
        let mut token = Token {
            value,
            pos_start,
            pos_end: pos_end.clone(),
        };
        if token.pos_start.is_some() {
            token.pos_end = token.pos_start.clone();
            token.pos_end.as_mut().unwrap().advance(None);
        }
        if pos_end.is_some() {
            token.pos_end = pos_end;
        }
        return token;
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub index: isize,
    pub ln: usize,
    pub col: isize,
    pub filename: String,
    pub text: String,
}

impl Position {
    pub fn new(filename: String, text: String) -> Position {
        Position {
            index: -1,
            ln: 0,
            col: -1,
            filename,
            text,
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

#[derive(Debug, Clone)]
pub struct Lexer {
    pub filename: String,
    pub text: String,
    pub pos: Position,
    pub current_char: Option<char>,
}

impl Lexer {
    pub fn new(filename: String, text: String) -> Lexer {
        Lexer {
            filename: filename.clone(),
            text:text.clone(),
            pos: Position::new(filename.clone().to_owned(), text.clone().to_owned()),
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

    fn make_number(& mut self) -> Token {
        let mut str = String::new();
        let mut dot_count: usize = 0;

        for (i, c) in self.text.clone().char_indices().skip(self.pos.index as usize) {
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

        return if dot_count == 0 {
            Token::new(TokenValue::INT(str.parse().unwrap()), Some(self.pos.clone()), None)
        } else {
            Token::new(
                TokenValue::FLOAT(str.parse().unwrap()),
                Some(self.pos.clone()),
                None,
            )
        };
    }

    pub fn make_tokens(& mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        self.advance();
        while self.current_char.is_some() {
            match self.current_char.unwrap() {
                '\t' | ' ' | '\n' | '\r' => self.advance(),
                '+' => {
                    tokens.push(Token::new(TokenValue::PLUS, None, None));
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::new(TokenValue::MINUS, None, None));
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::new(TokenValue::MULTIPLY, None, None));
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::new(TokenValue::DIVIDE, None, None));
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::new(TokenValue::LPARENT, None, None));
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::new(TokenValue::RPARENT, None, None));
                    self.advance();
                }
                '{' => {
                    tokens.push(Token::new(TokenValue::LBRACKET, None, None));
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::new(TokenValue::RBRACKET, None, None));
                    self.advance();
                }
                _ => {
                    if DIGITS.contains(self.current_char.unwrap().to_string().as_str()) {
                        tokens.push(self.make_number());
                    } else {
                        let pos_start = self.pos.clone();
                        let char = self.current_char.unwrap();
                        self.advance();
                        return Err(LexerError::IllegalCharError {
                            char,
                            data: DataError {
                                text: self.text.clone(),
                                filename: self.filename.clone(),
                                pos_start,
                                pos_end: self.pos.clone(),
                            },
                        });
                    }
                }
            }
        }
        return Ok(tokens);
    }
}

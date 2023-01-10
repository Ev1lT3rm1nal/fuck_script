// Replace always f64 for f128

use crate::errors::LexerError;
use crate::lexer;
use crate::lexer::{TokenValue};

#[derive(Debug, Clone)]
pub enum NumberNode {
    Int(isize),
    Float(f64),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BinOp {
    left: Box<OpSum>,
    right: Box<OpSum>,
    op: TokenValue,
}

#[derive(Debug, Clone)]
pub enum OpSum {
    OpSum(Box<OpSum>),
    BinOp(BinOp),
    NumberNode(NumberNode),
    Null,
}

pub fn parse(tokens: Vec<lexer::Token>) -> OpSum {
    let mut parser = Parser::new(tokens);
    return parser.parse();
}

#[derive(Debug, Clone)]
struct ParseResult {
    node: Option<lexer::Token>,
    error: Option<LexerError>,
}

enum RegisterData {
    ParseResult(ParseResult),
    Token(Option<lexer::Token>),
}

impl ParseResult {
    fn new() -> ParseResult {
        return ParseResult {
            node: None,
            error: None,
        };
    }

    fn register(&mut self, res: RegisterData) -> RegisterData {
        match res {
            RegisterData::ParseResult(res) => {
                if res.error.is_some() {
                    self.error = res.error;
                }
                return RegisterData::Token(res.node);
            }
            RegisterData::Token(res) => {
                return RegisterData::Token(res);
            }
        }
    }

    fn success(&mut self, node: lexer::Token)  {
        self.node = Some(node);
    }

    fn failure(&mut self, error: LexerError) {
        self.error = Some(error);
    }
    
}

pub struct Parser {
    tokens: Vec<lexer::Token>,
    current_token: Option<lexer::Token>,
    token_index: isize,
}

impl<'a> Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Parser {
        return Parser {
            tokens,
            current_token: None,
            token_index: -1,
        };
    }

    fn advance(&mut self) -> Option<lexer::Token> {
        self.token_index += 1;
        if 0 <= self.token_index && self.token_index < self.tokens.len() as isize {
            self.current_token = Some(self.tokens[self.token_index as usize].clone());
        }
        return self.current_token.clone();
    }

    fn factor(&mut self) -> OpSum {
        let token0 = self.current_token.clone();
        if token0.is_none() {
            return OpSum::Null;
        }
        let token = token0.unwrap();
        return match token.value {
            TokenValue::FLOAT(value) => {
                self.advance();
                OpSum::NumberNode(NumberNode::Float(value))
            }
            TokenValue::INT(value) => {
                self.advance();
                OpSum::NumberNode(NumberNode::Int(value))
            }
            _ => {
                self.advance();
                OpSum::Null
            }
        }
    }

    fn bin_op(&mut self, func: fn(&mut Parser) -> OpSum, ops: Vec<TokenValue>) -> OpSum {
        let mut left: OpSum = func(self);

        while self.current_token.is_some()
            && ops.contains(&self.current_token.clone().unwrap().value)
        {
            let op_token = self.current_token.clone().unwrap();
            self.advance();
            let right: OpSum = func(self);
            // left = OpSum{ .opSum = BinOp{ .left = left, .right = right, .op = op_token } };
            left = OpSum::BinOp(BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op: op_token.value,
            });
        }
        return left;
    }

    pub fn term(&mut self) -> OpSum {
        return self.bin_op(
            Parser::factor,
            vec![TokenValue::MULTIPLY, TokenValue::DIVIDE],
        );
    }

    pub fn expr(&mut self) -> OpSum {
        return self.bin_op(
            Parser::term,
            vec![TokenValue::PLUS, TokenValue::MINUS],
        );
    }

    pub fn parse(&mut self) -> OpSum {
        self.advance();
        return self.expr();
    }
}


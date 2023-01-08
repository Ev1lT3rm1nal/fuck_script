// Replace always f64 for f128

use crate::lexer;
use crate::lexer::{TokenValue};

#[derive(Debug)]
pub enum NumberNode {
    Int(isize),
    Float(f64),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct BinOp {
    left: Box<OpSum>,
    right: Box<OpSum>,
    op: lexer::TokenValue,
}

#[derive(Debug)]
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

pub struct Parser {
    tokens: Vec<lexer::Token>,
    current_token: Option<lexer::Token>,
    token_index: isize,
}

impl Parser {
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

    fn bin_op(&mut self, func: fn(&mut Parser) -> OpSum, ops: Vec<lexer::TokenValue>) -> OpSum {
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
            vec![lexer::TokenValue::MULTIPLY, lexer::TokenValue::DIVIDE],
        );
    }

    pub fn expr(&mut self) -> OpSum {
        return self.bin_op(
            Parser::term,
            vec![lexer::TokenValue::PLUS, lexer::TokenValue::MINUS],
        );
    }

    pub fn parse(&mut self) -> OpSum {
        self.advance();
        return self.expr();
    }
}

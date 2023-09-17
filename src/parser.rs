use anyhow::Result;
use anyhow::anyhow;

use crate::tokenizer::Token;

pub enum Expression {
}

pub struct Parser<'a> {
    input: &'a [Token],
    index: usize,
}

impl <'a> Parser<'a> {
    fn new(input: &'a [Token]) -> Self {
        Parser { 
            input, 
            index: 0
        }
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        todo!("parse_expression is not implemented yet!")
    }
}

pub fn parse_expression(tokens: &[Token]) -> Result<Expression> {
    Parser::new(tokens).parse_expression()
}

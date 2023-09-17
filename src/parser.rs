use anyhow::Result;
use anyhow::anyhow;

use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Expression {
    Number(i32),
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
        let token = self.input.get(self.index)
            .ok_or(anyhow!("Could not get token").context("parsing expression"))?;

        match token {
            Token::Number(number) => Ok(Expression::Number(*number)),
            token => todo!("parse_expression is not implemented for {:?} yet", token),
        }
    }
}

pub fn parse_expression(tokens: &[Token]) -> Result<Expression> {
    Parser::new(tokens).parse_expression()
}

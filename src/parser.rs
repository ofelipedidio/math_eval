use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;

use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Number(i32),
    Identifier(String),
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
        self.parse_expression_number()
            .or_else(|err| self.parse_expression_identifier().context(err))
    }

    fn parse_expression_number(&mut self) -> Result<Expression> {
        let token = self.input.get(self.index)
            .ok_or(anyhow!("Could not get token").context("parsing number"))?;

        match token {
            Token::Number(number) => Ok(Expression::Number(*number)),
            token => Err(anyhow!("Expected number, found {:?}", token))
        }
    }

    fn parse_expression_identifier(&mut self) -> Result<Expression> {
        let token = self.input.get(self.index)
            .ok_or(anyhow!("Could not get token").context("parsing identifier"))?;

        match token {
            Token::Identifier(identifier) => Ok(Expression::Identifier(identifier.clone())),
            token => Err(anyhow!("Expected identifier, found {:?}", token))
        }
    }
}

pub fn parse_expression(tokens: &[Token]) -> Result<Expression> {
    Parser::new(tokens).parse_expression()
}

#[cfg(test)]
mod test {
    use super::parse_expression;
    use super::Expression;
    use crate::tokenizer::Token;

    #[test]
    fn test_parser_number() {
        assert_eq!(parse_expression(&vec![Token::Number(0)]).ok(), Some(Expression::Number(0)));
        assert_eq!(parse_expression(&vec![Token::Number(123)]).ok(), Some(Expression::Number(123)));
    }
}

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;

use crate::tokenizer::Token;

macro_rules! expect {
    ($self:expr, $token:pat => $value:expr, $error:expr, $context:expr) => {
        match $self.input.get($self.index).ok_or(anyhow!("Expected {}, found EOF", $error).context($context))? {
            $token => {
                $self.index += 1;
                Ok($value)
            },
            token => Err(anyhow!("Expected {}, found {:?}", $error, token).context($context)),
        }
    };
}

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
        expect!(self, Token::Number(number) => Expression::Number(number.clone()), "number", "Parsing number")
    }

    fn parse_expression_identifier(&mut self) -> Result<Expression> {
        expect!(self, Token::Identifier(identifier) => Expression::Identifier(identifier.clone()), "identifier", "Parsing identifier")
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

    #[test]
    fn test_parser_identifier() {
        assert_eq!(parse_expression(&vec![Token::Identifier("x".to_string())]).ok(), Some(Expression::Identifier("x".to_string())));
        assert_eq!(parse_expression(&vec![Token::Identifier("identifier".to_string())]).ok(), Some(Expression::Identifier("identifier".to_string())));
    }
}

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

macro_rules! safeguard {
    ($value:expr, $block:expr) => {
        {
            let value = $value;
            (|| $block)().or_else(|e| {
                $value = value;
                Err(e)
            })
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Number(i32),
    Identifier(String),
    Add(Box<Expression>, Box<Expression>),
    Where(Box<Expression>, String, Box<Expression>)
}

enum FollowingExpression {
    Add(Expression),
    Where(String, Expression),
}

impl FollowingExpression {
    fn attach_expression(self, expression: Expression) -> Expression {
        match self {
            FollowingExpression::Add(e) => Expression::Add(
                Box::new(expression),
                Box::new(e)),
            FollowingExpression::Where(id, e) => Expression::Where(
                Box::new(expression),
                id,
                Box::new(e)),
        }
    }
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
        let base_expression = self.parse_expression_number()
            .or_else(|err| self.parse_expression_identifier().context(err));

        self.parse_following_expression(base_expression?)
    }

    fn parse_expression_number(&mut self) -> Result<Expression> {
        expect!(self, Token::Number(number) => Expression::Number(number.clone()), "number", "Parsing number")
    }

    fn parse_expression_identifier(&mut self) -> Result<Expression> {
        expect!(self, Token::Identifier(identifier) => Expression::Identifier(identifier.clone()), "identifier", "Parsing identifier")
    }

    fn parse_following_expression(&mut self, base_expression: Expression) -> Result<Expression> {
        let following_expression = self.parse_addition()
            .or_else(|err| self.parse_where_clause().context(err));

        match following_expression {
            Ok(expression) => Ok(expression.attach_expression(base_expression)),
            _ => Ok(base_expression)
        }
    }

    fn parse_addition(&mut self) -> Result<FollowingExpression> {
        safeguard!(self.index, {
            expect!(self, Token::Plus => (), "'+'", "parsing fillowing expression (addition)")?;
            let expr = self.parse_expression()?;
            Ok(FollowingExpression::Add(expr))
        })
    }

    fn parse_where_clause(&mut self) -> Result<FollowingExpression> {
        safeguard!(self.index, {
            expect!(self, Token::Comma => (), "','", "parsing fillowing expression (where clause)")?;
            expect!(self, Token::Where => (), "'where'", "parsing fillowing expression (where clause)")?;
            let id = expect!(self, Token::Identifier(id) => id, "','", "parsing fillowing expression (where clause)")?;
            expect!(self, Token::Equals => (), "'='", "parsing fillowing expression (where clause)")?;
            let expr = self.parse_expression()?;
            Ok(FollowingExpression::Where(id.clone(), expr))
        })
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

    #[test]
    fn test_parser_following() {
        assert_eq!(parse_expression(&vec![Token::Number(1), Token::Plus, Token::Number(1)]).ok(), Some(Expression::Add(Box::new(Expression::Number(1)), Box::new(Expression::Number(1)))));
        assert_eq!(
            parse_expression(&vec![Token::Identifier("x".to_string()), Token::Comma, Token::Where, Token::Identifier("x".to_string()), Token::Equals, Token::Number(1)]).ok(),
            Some(Expression::Where(Box::new(Expression::Identifier("x".to_string())), "x".to_string(), Box::new(Expression::Number(1)))));
    }
}

use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number(i32),

    Comma,
    Equals,
    Plus,
    LeftParenthesis,
    RightParenthesis,

    Identifier(String),
    Where,
}

pub struct TokenStream<'a> {
    input: &'a [char],
    index: usize,
}

impl <'a> TokenStream<'a> {
    fn new(input: &'a [char]) -> Self {
        TokenStream {
            input,
            index: 0,
        }
    }

    fn parse_number(&mut self) -> Result<Token> {
        let mut number = 0;

        loop {
            match self.input.get(self.index) {
                Some(c @ '0'..='9') => {
                    number = (number * 10) + ((*c as i32) - ('0' as i32));
                    self.index += 1;
                }
                _ => break,
            }
        }

        Ok(Token::Number(number))
    }

    fn parse_identifier(&mut self) -> Result<Token> {
        let mut identifier = String::new();

        match self.input.get(self.index) {
            Some(c @ ('a'..='z' | 'A'..='Z' | '_')) => {
                identifier.push(*c);
                self.index += 1;
            }
            Some(c) => return Err(anyhow!("Expect IDENTIFIER_STARTED, found {:?} while parsing an identifier", c)),
            None => return Err(anyhow!("Expect IDENTIFIER_STARTED, found EOF while parsing an identifier")),
        }

        loop {
            match self.input.get(self.index) {
                Some(c @('a'..='z' | 'A'..='Z' | '_' | '0'..='9')) => {
                    identifier.push(*c);
                    self.index += 1;
                }
                _ => break,
            }
        }

        match identifier.as_str() {
            "where" => Ok(Token::Where),
            _ => Ok(Token::Identifier(identifier)),
        }
    }
}

impl <'a> Iterator for TokenStream<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.input.get(self.index) {
                Some(' ' | '\t' | '\r' | '\n') => self.index += 1,
                _ => break,
            }
        }

        match self.input.get(self.index)? {
            ',' => {self.index += 1; Some(Ok(Token::Comma))},
            '=' => {self.index += 1; Some(Ok(Token::Equals))},
            '+' => {self.index += 1; Some(Ok(Token::Plus))},
            '(' => {self.index += 1; Some(Ok(Token::LeftParenthesis))},
            ')' => {self.index += 1; Some(Ok(Token::RightParenthesis))},
            '0'..='9' => Some(self.parse_number()),
            'a'..='z' | 'A'..='Z' | '_' => Some(self.parse_identifier()),
            c => Some(Err(anyhow!("Unexpected character {:?} found while parsing a token", c))),
        }
    }
}

pub fn tokenize<'a>(input: &'a [char]) -> TokenStream<'a> {
    TokenStream::new(input)
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use super::tokenize;
    use super::Token;

    fn tokenize_str(str: &str) -> Result<Vec<Token>> {
        tokenize(&str.chars().collect::<Vec<_>>()).collect()
    }

    #[test]
    fn test_basic_tokens() {
        assert_eq!(tokenize_str("=").ok(), Some(vec![Token::Equals]));
        assert_eq!(tokenize_str("+").ok(), Some(vec![Token::Plus]));
        assert_eq!(tokenize_str(",").ok(), Some(vec![Token::Comma]));
        assert_eq!(tokenize_str("(").ok(), Some(vec![Token::LeftParenthesis]));
        assert_eq!(tokenize_str(")").ok(), Some(vec![Token::RightParenthesis]));
    }

    #[test]
    fn test_identifiers() {
        assert_eq!(tokenize_str("lower_case").ok(), Some(vec![Token::Identifier("lower_case".to_string())]));
        assert_eq!(tokenize_str("UPPER_CASE").ok(), Some(vec![Token::Identifier("UPPER_CASE".to_string())]));

        assert_eq!(tokenize_str("where").ok(), Some(vec![Token::Where]));
    }

    #[test]
    fn test_numbers() {
        assert_eq!(tokenize_str("0").ok(), Some(vec![Token::Number(0)]));
        assert_eq!(tokenize_str("5").ok(), Some(vec![Token::Number(5)]));
        assert_eq!(tokenize_str("10").ok(), Some(vec![Token::Number(10)]));
        assert_eq!(tokenize_str("21").ok(), Some(vec![Token::Number(21)]));
        assert_eq!(tokenize_str("12345").ok(), Some(vec![Token::Number(12345)]));
    }

    #[test]
    fn test_sequence() {
        assert_eq!(tokenize_str("+()").ok(), Some(vec![Token::Plus, Token::LeftParenthesis, Token::RightParenthesis]));
        assert_eq!(tokenize_str("1+x").ok(), Some(vec![Token::Number(1), Token::Plus, Token::Identifier("x".to_string())]));
    }
}


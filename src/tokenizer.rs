use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
pub enum Token {
    Number(i32),

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
    done: bool,
}

impl <'a> TokenStream<'a> {
    fn new(input: &'a [char]) -> Self {
        TokenStream {
            input,
            index: 0,
            done: false,
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
        if self.done {
            return None;
        }

        loop {
            match self.input.get(self.index) {
                Some(' ' | '\t' | '\r' | '\n') => self.index += 1,
                _ => break,
            }
        }

        match self.input.get(self.index)? {
            '=' => {self.index += 1; Some(Ok(Token::Equals))},
            '+' => {self.index += 1; Some(Ok(Token::Plus))},
            '(' => {self.index += 1; Some(Ok(Token::LeftParenthesis))},
            ')' => {self.index += 1; Some(Ok(Token::RightParenthesis))},
            '0'..='9' => Some(self.parse_number()),
            'a'..='z' | 'A'..='Z' | '_' => Some(self.parse_identifier()),
            c => Some(Err(anyhow!("Unexpected character {:?} found while parsing a token", c))),
        }.map(|x| x.or_else(|e| {
            self.done = true;
            Err(e)
        }))
    }
}

pub fn tokenize<'a>(input: &'a [char]) -> TokenStream<'a> {
    TokenStream::new(input)
}


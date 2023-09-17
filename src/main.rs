use std::fs;

mod tokenizer;
mod parser;

use anyhow::Result;

use crate::parser::parse_expression;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let chars: Vec<_> = input.chars().collect();
    
    let token_stream = tokenizer::tokenize(&chars);
    let tokens: Result<Vec<_>> = token_stream.collect();
    let tokens = tokens.unwrap();
    println!("{:?}", &tokens);

    let expression = parse_expression(&tokens).unwrap();
    println!("{:?}", &expression);
}

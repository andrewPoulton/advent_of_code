use std::fs;
use std::str::FromStr;

pub fn file2vec<T: FromStr>(filename: &String)->Vec<Result<T, T::Err>>{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents.split("\n")
    .map(|x: &str| x.parse::<T>())
    .collect()
} 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Atom(char),
    Op(char),
    Eof,
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' |
                'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c),
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }
    fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}
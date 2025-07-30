use super::token::Token;
use std::{iter::Peekable, str::Chars};

pub struct Tokenizer<'a> {
    expression: Peekable<Chars<'a>>,
    reached_end: bool,
    unexpected_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expression: &'a str) -> Self {
        Self {
            expression: expression.chars().peekable(),
            reached_end: false,
            unexpected_char: None,
        }
    }

    pub fn get_unexpected_char(&self) -> Option<char> {
        self.unexpected_char
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_end {
            return None;
        }
        let next_chr = self.expression.next();
        match next_chr {
            Some(chr) if chr.is_numeric() => {
                let mut number = String::from(chr);
                while let Some(next) = self.expression.peek() {
                    if next.is_numeric() {
                        number.push(self.expression.next().unwrap());
                    } else {
                        break;
                    }
                }

                Some(Token::Number(number.parse().unwrap()))
            }
            Some(chr) if chr.is_whitespace() => {
                while let Some(_) = self.expression.next_if(|c| c.is_whitespace()) {}
                self.next()
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => {
                self.reached_end = true;
                Some(Token::EOF)
            }
            Some(chr) => {
                self.unexpected_char = Some(chr);
                None
            }
        }
    }
}

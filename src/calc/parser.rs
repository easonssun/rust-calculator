use crate::calc::{
    ast::Node,
    error::{CalcError, CalcResult},
    token::{OperatorPrecedence, Token},
    tokenizer::{self, Tokenizer},
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expression: &'a str) -> CalcResult<Self> {
        let mut tokenizer = Tokenizer::new(expression);
        let current_token = tokenizer
            .next()
            .ok_or_else(|| CalcError::UnexpectedChar(tokenizer.get_unexpected_char().unwrap()))?;
        Ok(Self {
            tokenizer,
            current_token: current_token,
        })
    }

    pub fn parse(&mut self) -> CalcResult<Node> {
        todo!()
    }
}

impl<'a> Parser<'a> {
    fn next_token(&mut self) -> CalcResult<()> {
        self.current_token = self.tokenizer.next().ok_or_else(|| {
            CalcError::UnexpectedChar(self.tokenizer.get_unexpected_char().unwrap())
        })?;

        Ok(())
    }

    fn parse_expression(&mut self, precedence: OperatorPrecedence) -> CalcResult<Node> {
        let expr = self.parse_number_or_expression()?;
        todo!()
    }

    fn parse_number_or_expression(&mut self) -> CalcResult<Node> {
        match self.current_token {
            Token::Number(value) => {
                self.next_token()?;
                Ok(Node::Number(value))
            }
            Token::Subtract => {
                self.next_token()?;
                let expr = self.parse_expression(OperatorPrecedence::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::LeftParen => {
                self.next_token()?;
                let expr = self.parse_expression(OperatorPrecedence::Default)?;
                if self.current_token != Token::RightParen {
                    if self.current_token == Token::EOF {
                        return Err(CalcError::InvalidOperator(String::from("不完整的表达式")));
                    };
                    return Err(CalcError::InvalidOperator(format!(
                        "Expected ')', but got '{}'",
                        self.current_token
                    )));
                }
                self.next_token()?;
                Ok(expr)
            }
            _ => {
                if self.current_token == Token::EOF {
                    return Err(CalcError::InvalidOperator(String::from("不完整的表达式")));
                };
                return Err(CalcError::InvalidOperator(format!(
                    "Expected number or expression, but got '{}'",
                    self.current_token
                )));
            }
        }
    }
}

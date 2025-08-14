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
        self.parse_expression(OperatorPrecedence::Default)
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
        let mut expr = self.parse_number_or_expression()?;

        while precedence < self.current_token.get_precedence() {
            expr = self.parse_binary_expression(expr)?;
        }
        Ok(expr)
    }

    fn parse_binary_expression(&mut self, left_expr: Node) -> CalcResult<Node> {
        match self.current_token {
            Token::Add => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::AddOrSubtract)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::AddOrSubtract)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::MultiplyOrDivide)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::MultiplyOrDivide)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::Power)?;
                Ok(Node::Power(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => unreachable!("Invalid operator")
        }
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

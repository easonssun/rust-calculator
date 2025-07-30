use std::fmt::Display;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Add,         // +
    Subtract,    // -
    Multiply,    // *
    Divide,      // /
    Caret,       // ^
    LeftParen,   // (
    RightParen,  // )
    Number(f64), // number
    EOF,         // success end
}

impl Token {
    pub fn get_precedence(&self) -> OperatorPrecedence {
        use OperatorPrecedence::*;
        use Token::*;
        match self {
            Add | Subtract => AddOrSubtract,
            Multiply | Divide => MultiplyOrDivide,
            Caret => Power,
            _ => Default,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            Add => write!(f, "+"),
            Subtract => write!(f, "-"),
            Multiply => write!(f, "*"),
            Divide => write!(f, "/"),
            Caret => write!(f, "^"),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            Number(n) => write!(f, "{}", n),
            EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum OperatorPrecedence {
    Default,
    AddOrSubtract,
    MultiplyOrDivide,
    Power,
    Negative,
}

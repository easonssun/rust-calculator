use rust_decimal::Decimal;

use crate::calc::{error::CalcResult, parser::Parser};

mod ast;
mod error;
mod parser;
mod token;
mod tokenizer;

pub fn calc(expression: &str) -> CalcResult<Decimal> {
    let mut parser = Parser::new(expression)?;
    let ast = parser.parse()?;
    Ok(ast.eval())
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(calc("1+2").unwrap(), dec!(3));
        assert_eq!(calc("1+2*3").unwrap(), dec!(7));
        assert_eq!(calc("1+2*3/4").unwrap(), dec!(2.5));
        assert_eq!(calc("-1^2").unwrap(), dec!(1));
        assert_eq!(calc("3- (2+3) * 2 -1 *(-3*3)").unwrap(), dec!(2));
    }
}

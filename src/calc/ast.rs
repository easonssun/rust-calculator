use rust_decimal::{Decimal, MathematicalOps};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Power(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(Decimal),
}

impl Node {
    pub fn eval(&self) -> Decimal {
        use Node::*;
        match self {
            Add(left, right) => left.eval() + right.eval(),
            Subtract(left, right) => left.eval() - right.eval(),
            Multiply(left, right) => left.eval() * right.eval(),
            Divide(left, right) => left.eval() / right.eval(),
            Power(left, right) => left.eval().powd(right.eval()),
            Negative(node) => -node.eval(),
            Number(num) => *num,
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(Node::Number(dec!(1)).eval(), dec!(1));
        assert_eq!(
            Node::Negative(Box::new(Node::Number(dec!(1)))).eval(),
            dec!(-1)
        );
        assert_eq!(
            Node::Add(
                Box::new(Node::Number(dec!(1))),
                Box::new(Node::Number(dec!(2)))
            )
            .eval(),
            dec!(3)
        );
        assert_eq!(
            Node::Subtract(
                Box::new(Node::Number(dec!(1))),
                Box::new(Node::Number(dec!(2)))
            )
            .eval(),
            dec!(-1)
        );
        assert_eq!(
            Node::Multiply(
                Box::new(Node::Number(Decimal::from(1))),
                Box::new(Node::Number(Decimal::from(2)))
            )
            .eval(),
            dec!(2)
        );
        assert_eq!(
            Node::Divide(
                Box::new(Node::Number(dec!(1))),
                Box::new(Node::Number(dec!(2)))
            )
            .eval(),
            dec!(0.5)
        );
        assert_eq!(
            Node::Power(
                Box::new(Node::Number(dec!(2))),
                Box::new(Node::Number(dec!(3)))
            )
            .eval(),
            dec!(8)
        )
    }
}

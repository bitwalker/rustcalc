use std::fmt::{self, Display, Formatter};

enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent
}

pub fn parse(expression: &str) -> Result<String, ParseError> {
    let chars = expression.chars();
    return String::new();
}

pub enum ParseError {
    InvalidOperator(str),
    InvalidExpression
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::ParseError::*;
        match *self {
            InvalidOperator(ref op) => write!(f, "Invalid operator: {}", op),
            InvalidExpression => write!(f, "Invalid expression.")
        }
    }
}

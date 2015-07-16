use std::fmt;
use parser_combinators::*;
use parser_combinators::primitives::{State, Stream};

// Main entry point for the parser
pub fn parse(input: &str) -> Result<(Expr, &str), ParseError<char>> {
    return parser(term).parse(input);
}

// The type which defines our expression primitives (numbers and operators basically)
#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>)
}
// For pretty printing, we implement the Display trait for Expr,
// currently we're not pretty printing the parsed expression tree,
// but we could easily if we wanted to thanks to this
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Number(ref i)       => write!(f, "{}", i),
            Expr::Plus(ref l, ref r)  => write!(f, "{} + {}", l, r),
            Expr::Minus(ref l, ref r) => write!(f, "{} - {}", l, r),
            Expr::Mul(ref l, ref r)   => write!(f, "{} * {}", l, r),
            Expr::Div(ref l, ref r)   => write!(f, "{} / {}", l, r),
            Expr::Pow(ref l, ref r)   => write!(f, "{}^{}", l, r)
        }
    }
}

// Parser which extracts a signed integer or floating point value from the input string
fn number<'a, I>(input: State<I>) -> ParseResult<f64, I>
    where I: Stream<Item=char> {
    let (((negation, whole), fractional), input) = try!(optional(char('-'))
                            .and(many1::<String, _>(digit()))
                            .and(optional(char('.').and(many1::<String, _>(digit()))))
                            .expected("number")
                            .parse_state(input));

    // Build whole number
    let mut n = 0;
    for c in whole.chars() {
        n = n * 10 + (c as i64 - '0' as i64)
    }
    // Build fractional part
    let mut m = 0;
    let mut precision = 0;
    match fractional {
        Some((_, f)) => {
            for c in f.chars() {
                precision = precision + 1u32;
                m = m * 10 + (c as i64 - '0' as i64);
            }
        }
        None => ()
    }
    // Add fractional part to whole if present
    let mut number = n as f64;
    if precision > 0 {
        number = number + (m as f64 / (10i64.pow(precision) as f64));
    }
    number = match negation {
        Some(_) => number * -1f64,
        None    => number
    };
    Ok((number, input))
}

// Parser which extracts an expression from the input string, where an expression
// is any signed integer or parenthized sub-expression, with whitespace ignored.
#[allow(unconditional_recursion)]
fn expr(input: State<&str>) -> ParseResult<Expr, &str> {
    let spaces     = spaces();
    let number     = parser(number);
    let paren_expr = between(char('('), char(')'), parser(term)).expected("(");

    spaces.clone()
          .with(number.map(Expr::Number).or(paren_expr))
          .skip(spaces)
          .parse_state(input)
}

// Parser which extracts terms, where a term is an operator surrounded by two operands.
fn term(input: State<&str>) -> ParseResult<Expr, &str> {
    fn pow(l: Expr, r: Expr)   -> Expr { Expr::Pow(Box::new(l), Box::new(r)) }
    fn mul(l: Expr, r: Expr)   -> Expr { Expr::Mul(Box::new(l), Box::new(r)) }
    fn div(l: Expr, r: Expr)   -> Expr { Expr::Div(Box::new(l), Box::new(r)) }
    fn plus(l: Expr, r: Expr)  -> Expr { Expr::Plus(Box::new(l), Box::new(r)) }
    fn minus(l: Expr, r: Expr) -> Expr { Expr::Minus(Box::new(l), Box::new(r)) }

    let exp      = char('^').map(|_| pow);
    let multiply = char('*').map(|_| mul);
    let divide   = char('/').map(|_| div);
    let add      = char('+').map(|_| plus);
    let subtract = char('-').map(|_| minus);
    return chainl1(chainl1(chainl1(chainl1(chainl1(parser(expr), exp), multiply), divide), add), subtract)
        .parse_state(input);
}

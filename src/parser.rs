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
    Int(i64),
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
            Expr::Int(ref i)          => write!(f, "{}", i),
            Expr::Plus(ref l, ref r)  => write!(f, "{} + {}", l, r),
            Expr::Minus(ref l, ref r) => write!(f, "{} - {}", l, r),
            Expr::Mul(ref l, ref r)   => write!(f, "{} * {}", l, r),
            Expr::Div(ref l, ref r)   => write!(f, "{} / {}", l, r),
            Expr::Pow(ref l, ref r)   => write!(f, "{}^{}", l, r)
        }
    }
}

// Parser which extracts a signed integer value from the input string
fn integer<'a, I>(input: State<I>) -> ParseResult<i64, I>
    where I: Stream<Item=char> {
    let ((negation, s), input) = try!(optional(char('-'))
                            .and(many1::<String, _>(digit()))
                            .expected("integer")
                            .parse_state(input));
    let mut n = 0;
    for c in s.chars() {
        n = n * 10 + (c as i64 - '0' as i64)
    }
    n = match negation {
        Some(_) => n * -1,
        None    => n
    };
    Ok((n, input))
}

// Parser which extracts an expression from the input string, where an expression
// is any signed integer or parenthized sub-expression, with whitespace ignored.
#[allow(unconditional_recursion)]
fn expr(input: State<&str>) -> ParseResult<Expr, &str> {
    let spaces     = spaces();
    let integer    = parser(integer);
    let paren_expr = between(char('('), char(')'), parser(term)).expected("(");

    spaces.clone()
          .with(integer.map(Expr::Int).or(paren_expr))
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

// We're using unstable features for the Box<T> destructuring in calculator::evaluate,
// we could do it without them, but it's much cleaner this way. Downside is you have to
// compile with the nightly compiler for now :(
#![feature(box_syntax, box_patterns)]

extern crate parser_combinators;
use parser_combinators::ParseError;

mod parser;
mod calculator;

pub fn calc(input: &str) -> Result<i64, ParseError<char>> {
    // Parses the input string to an expression tree
    let result: Result<(parser::Expr, &str), ParseError<char>> = parser::parse(input);
    match result {
        // If successful, the expression tree is evaluated and the result returned
        Ok((expression_tree, ignored)) => {
            // The parser will fail if the input does not start with a valid expression,
            // but if the expression contains invalid input that does not result in an
            // invalid expression being parsed up to that point, it will be ignored, and
            // we get it back here to be printed to the user so they are aware of what happened.
            match ignored {
                "" => (),
                _  => println!("Ignored extraneous input: {}", ignored)
            }
            Ok(calculator::evaluate(&expression_tree))
        }
        Err(e) => Err(e)
    }
}

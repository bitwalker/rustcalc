use parser::Expr;
use parser::Expr::*;

// The main entry point for the calculator itself.
// It receives an expression tree, and recursively reduces it to a single signed 64-bit integer
pub fn evaluate(expr: &Expr) -> f64 {
    match expr {
        &Number(ref i)               => *i,
        &Plus(box ref l, box ref r)  => evaluate(&l) + evaluate(&r),
        &Minus(box ref l, box ref r) => evaluate(&l) - evaluate(&r),
        &Mul(box ref l, box ref r)   => evaluate(&l) * evaluate(&r),
        &Pow(box ref l, box ref r)   => evaluate(&l).powf(evaluate(&r)),
        &Div(box ref l, box ref r)   => {
            let numerator = evaluate(&l);
            let divisor   = evaluate(&r);
            match divisor {
                n if n == 0f64 => 0f64,
                _              => numerator / divisor
            }
        }
    }
}

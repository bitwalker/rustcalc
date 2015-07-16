extern crate rustcalc;

#[test]
fn simple_addition() {
    let input = "2+2";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(4f64, x),
        _     => assert!(false)
    }
}

#[test]
fn complex_addition() {
    let input = "2+ 3 + (-8)";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(-3f64, x),
        _     => assert!(false)
    }
}

#[test]
fn simple_subtraction() {
    let input = "3-2";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(1f64, x),
        _     => assert!(false)
    }
}

#[test]
fn complex_subtraction() {
    let input = "3--2";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(5f64, x),
        _     => assert!(false)
    }
}

#[test]
fn simple_multiplication() {
    let input = "2*2";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(4f64, x),
        _     => assert!(false)
    }
}

#[test]
fn complex_multiplication() {
    let input = "2*-2*6";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(-24f64, x),
        _     => assert!(false)
    }
}

#[test]
fn simple_division() {
    let input = "4/2";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(2f64, x),
        _     => assert!(false)
    }
}

#[test]
fn complex_division() {
    let input = "4/2/-1";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(-2f64, x),
        _     => assert!(false)
    }
}

#[test]
fn simple_exponentiation() {
    let input = "2^4";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(16f64, x),
        _     => assert!(false)
    }
}

#[test]
fn complex_exponentiation() {
    let input = "2^2^(4/2)";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(16f64, x),
        _     => assert!(false)
    }
}

#[test]
fn integration_test() {
    let input = "2 - 5 + 323948234 / 2 ^ (1 * 2)";
    match rustcalc::calc(input) {
        Ok(x) => assert_eq!(-80987061.5, x),
        _     => assert!(false)
    }
}

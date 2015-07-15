#![feature(test)]

extern crate rustcalc;
extern crate test;
use test::Bencher;

#[bench]
fn benchmark_rustcalc(b: &mut Bencher) {
    let inputs = [
        "2+2", "2+ 3 + (-8)", "3-2", "3--2",
        "2*2", "2*-2*6", "4/2", "4/2/-1",
        "2^4", "2^2^(4/2)",
        "2 - 5 + 323948234 / 2 ^ (1 * 2)"
    ];
    b.iter(|| {
        let mut result = false;
        for input in inputs.iter() {
            match rustcalc::calc(input) {
                Ok(_) => result = true,
                _     => result = false
            }
        }
        result
    });
}

## rustcalc

A tiny command line calculator built in Rust for a code challenge.

It supports `+`, `-`, `/`, `*`, and `^` operators, as well as the
usage of `-` as negation. Grouping with `()` is also allowed. Numbers
can either be integers or arbitrary-precision floats.

## Example

```bash
$ ./bin/rustcalc
Welcome to rustcalc!
To quit, just type `exit` and press ENTER.
Type an expression and press ENTER to evaluate: 3.5+(2-1)*(10^2)/2
Result: 3.5+(2-1)*(10^2)/2 == 53.5
Type an expression and press ENTER to evaluate: exit
$
```

## Usage (from binary)

1. Download `bin/rustcalc` or `bin/rustcalc.exe`
2. From the command line: `./rustcalc` or `.\rustcalc.exe`

## Usage (from source)

1. Install Rust (nightly is required for some feature flags in use)
2. Clone rustcalc repository
3. From the command line: `cd rustcalc && cargo run`

## Benchmarks

1. Install Rust (nightly)
2. Clone rustcalc repository
3. From the command line: `cd rustcalc && cargo bench`

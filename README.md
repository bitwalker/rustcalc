## rustcalc

A tiny command line calculator built in Rust for a code challenge.

It supports `+`, `-`, `/`, `*`, and `^` operators, as well as the
usage of `-` as negation. It does not currently work on floating
point numbers, and division results in truncated integer values, so
there is an improvement to be made there once I sort out how I want to
handle that in the parser.

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

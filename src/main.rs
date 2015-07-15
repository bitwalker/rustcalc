use std::io;
use std::io::Write;

extern crate rustcalc;

fn main() {
    // Print intro
    println!("Welcome to rustcalc!");
    println!("To quit, just type `exit` and press ENTER.");

    // Begin program loop
    loop {
        print!("Type an expression and press ENTER to evaluate: ");
        // Ensure intro is flushed since the terminal is likely line-buffered
        io::stdout().flush().ok();

        // Collect expression to evaluate
        let mut expression = String::new();
        io::stdin().read_line(&mut expression)
                   .ok()
                   .expect("Failed to understand the provided expression. Try again.");

        match expression.trim() {
            ""      => continue,
            "exit"  => break,
            trimmed => {
                let num = rustcalc::calc(trimmed);
                match num {
                    Ok(x)    => println!("Result: {} == {}", trimmed, x),
                    Err(err) => println!("Invalid expression: {}", err)
                }
            }
        }
    }
}

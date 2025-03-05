use std::io;

use repl::start;

pub mod token;
pub mod lexer;
pub mod repl;

fn main() {
    println!("Welcome to the Tofu interpreter.");
    start(io::stdin(), io::stdout());
}

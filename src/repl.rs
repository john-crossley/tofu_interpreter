use std::io::{Stdin, Stdout, Write};

use crate::{lexer::Lexer, token::TokenKind};

pub fn start(stdin: Stdin, mut stdout: Stdout) {
    loop {
        write!(stdout, ">> ").expect("Uh-oh, failed to write.");
        stdout.flush().expect("Should have flushed stdout 🚽");

        let mut input = String::new();

        if let Err(e) = stdin.read_line(&mut input) {
            writeln!(stdout, "Error {e}").expect("Should have written error.");
            return;
        }

        let mut lexer = Lexer::new(&input);

        loop {
            let token = lexer.next();
            if token.kind == TokenKind::Eof {
                break;
            }
            writeln!(stdout, "{token:?}").expect("Should have written token.");
        }
    }
}

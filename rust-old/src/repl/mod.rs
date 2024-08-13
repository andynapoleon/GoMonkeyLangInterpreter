use crate::lexer::*;
use crate::token::*;
use std::io::{BufRead, Write};

const PROMPT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(mut scanner: R, mut out_writer: W) {
    loop {
        // Print the prompt
        write!(out_writer, "{}", PROMPT).unwrap();
        out_writer.flush().unwrap();

        // Read the next line
        let mut line = String::new();
        let bytes_read = scanner.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            return;
        }

        // Tokenization
        let mut lexer = Lexer::new(line);
        loop {
            let tok = lexer.next_token();
            if tok.token_type == TokenType::Eof {
                break;
            }
            writeln!(out_writer, "{:?}", tok).unwrap();
        }
    }
}

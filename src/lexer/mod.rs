use crate::token::*;

pub struct Lexer {
    pub input: String,
    pub position: usize, // current position in input (points to current char)
    pub read_position: usize, // current reading position in input (after current char)
    pub ch: Option<char>, // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
    }
}

#[cfg(test)]
mod tests {
    // Include names from outer scope
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){}";

        // A vector of tokens
        let mut tests: Vec<Token> = Vec::new();
        tests.push(Token {
            token_type: TokenType::Assign,
            literal: "=".to_owned(),
        });
        tests.push(Token {
            token_type: TokenType::Plus,
            literal: "+".to_owned(),
        });
        tests.push(Token {
            token_type: TokenType::Lparen,
            literal: "(".to_owned(),
        });
        tests.push(Token {
            token_type: TokenType::Rparen,
            literal: ")".to_owned(),
        });
        tests.push(Token {
            token_type: TokenType::Lbrace,
            literal: "{".to_owned(),
        });
        tests.push(Token {
            token_type: TokenType::Rbrace,
            literal: "}".to_owned(),
        });
        tests.push(Token {
            token_type: TokenType::Coma,
            literal: ",".to_owned(),
        });
        tests.push(Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_owned(),
        });
        tests.push(Token {
            token_type: TokenType::Eof,
            literal: "".to_owned(),
        });
    }
}

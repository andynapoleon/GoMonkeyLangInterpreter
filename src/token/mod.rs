// src/token/mod.rs

// Define TokenType as an enum
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    // Add more token types as needed.
}

// Define Token struct
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

// Implement a constructor for Token
impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

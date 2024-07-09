use std::collections::HashMap;
use token::TokenType;

/**  Lexer functions */

// Check if a char is an ASCII letter
pub fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

// Check if a char is a digit
pub fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

// Check and return if an string is an ident or a keyword, and which one
pub fn look_up_ident(ident: String) -> TokenType {
    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("fn", TokenType::Function),
        ("let", TokenType::Let),
        ("true", TokenType::True),
        ("false", TokenType::False),
        ("if", TokenType::If),
        ("else", TokenType::Else),
        ("return", TokenType::Return),
    ]);
    if let Some(&tok) = keywords.get(ident.as_str()) {
        tok
    } else {
        TokenType::Ident
    }
}

// Declare the modules
pub mod lexer;
pub mod token;

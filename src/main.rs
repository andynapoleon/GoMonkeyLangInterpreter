// Import the modules
mod lexer;
mod token;

use token::{Token, TokenType};

fn main() {
    // Create a token
    let token = Token::new(TokenType::Ident, "example".to_string());
    // Example token comparison
    let another_token = Token::new(TokenType::Ident, "example".to_string());
    println!("Tokens are equal: {}", token == another_token);
}

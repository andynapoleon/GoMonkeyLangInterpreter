// Define TokenType as an enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident,
    Int,
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

// Define Token struct
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

// Implement a constructor for Token
impl Token {
    pub fn new(token_type: TokenType, literal: Option<char>) -> Self {
        let literal_str = if let Some(literal_char) = literal {
            literal_char.to_string()
        } else {
            String::new()
        };
        Token {
            token_type,
            literal: literal_str,
        }
    }
}

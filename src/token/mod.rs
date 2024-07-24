// Define TokenType as an enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Default,
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
    Eq,
    NotEq,
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
    pub fn default() -> Self {
        Self {
            token_type: TokenType::Default,
            literal: "".to_owned(),
        }
    }
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

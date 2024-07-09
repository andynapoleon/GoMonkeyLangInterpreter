use crate::is_digit;
use crate::is_letter;
use crate::look_up_ident;
use crate::token::*;

// Lexer struct
#[derive(Debug)]
pub struct Lexer {
    pub input: String,
    pub position: usize, // current position in input (points to current char)
    pub read_position: usize, // current reading position in input (after current char)
    pub ch: Option<char>, // current char under examination
}

// Lexer's methods
impl Lexer {
    // Create a new lexer object
    pub fn new(input: String) -> Self {
        let first_char = input.chars().next();
        Self {
            input,
            position: 0,
            read_position: 1,
            ch: first_char,
        }
    }

    // Read the current char and move the cursor to the next one in the input string
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Traverse the input string and generate tokens
    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();
        let tok: Token = match self.ch {
            Some('=') => {
                if let Some(c) = self.peek_char() {
                    if c == '=' {
                        self.read_char();
                        Token {
                            token_type: TokenType::Eq,
                            literal: "==".to_owned(),
                        }
                    // if next char is some other char
                    } else {
                        Token::new(TokenType::Assign, self.ch)
                    }
                // if next char is None
                } else {
                    Token::new(TokenType::Assign, self.ch)
                }
            }
            Some(';') => Token::new(TokenType::Semicolon, self.ch),
            Some('(') => Token::new(TokenType::Lparen, self.ch),
            Some(')') => Token::new(TokenType::Rparen, self.ch),
            Some(',') => Token::new(TokenType::Comma, self.ch),
            Some('+') => Token::new(TokenType::Plus, self.ch),
            Some('{') => Token::new(TokenType::Lbrace, self.ch),
            Some('}') => Token::new(TokenType::Rbrace, self.ch),
            Some('-') => Token::new(TokenType::Minus, self.ch),
            Some('!') => {
                if let Some(c) = self.peek_char() {
                    if c == '=' {
                        self.read_char();
                        Token {
                            token_type: TokenType::NotEq,
                            literal: "!=".to_owned(),
                        }
                    // if next char is some other char
                    } else {
                        Token::new(TokenType::Bang, self.ch)
                    }
                // if next char is None
                } else {
                    Token::new(TokenType::Bang, self.ch)
                }
            }
            Some('*') => Token::new(TokenType::Asterisk, self.ch),
            Some('/') => Token::new(TokenType::Slash, self.ch),
            Some('<') => Token::new(TokenType::Lt, self.ch),
            Some('>') => Token::new(TokenType::Gt, self.ch),
            // ident + keyword case
            Some(c) => {
                if is_letter(c) {
                    let literal = self.read_identifier();
                    Token {
                        token_type: look_up_ident(literal.clone()),
                        literal,
                    }
                } else if is_digit(c) {
                    Token {
                        token_type: TokenType::Int,
                        literal: self.read_number(),
                    }
                } else {
                    Token::new(TokenType::Illegal, self.ch)
                }
            }
            // illegal char
            None => Token::new(TokenType::Eof, self.ch),
        };
        // if token is ident or keyword, no need to read char anymore since it's been done already
        if tok.token_type != TokenType::Function
            && tok.token_type != TokenType::Let
            && tok.token_type != TokenType::Int
            && tok.token_type != TokenType::Ident
            && tok.token_type != TokenType::True
            && tok.token_type != TokenType::False
            && tok.token_type != TokenType::If
            && tok.token_type != TokenType::Else
            && tok.token_type != TokenType::Return
        {
            self.read_char();
        }
        tok
    }

    // Read an ident or keyword
    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while let Some(c) = self.ch {
            if is_letter(c) {
                self.read_char()
            } else {
                break;
            }
        }
        self.input[position..self.position].to_owned()
    }

    // Read a number (integer)
    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while let Some(c) = self.ch {
            if is_digit(c) {
                self.read_char()
            } else {
                break;
            }
        }
        self.input[position..self.position].to_owned()
    }

    // Skip white space, newline, tab, and carriage return
    pub fn skip_white_space(&mut self) {
        while let Some(c) = self.ch {
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                self.read_char();
            } else {
                break;
            }
        }
    }

    // Look ahead into the input, but don't move the pointer
    pub fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        }
    }
}

#[cfg(test)]
mod tests {
    // Include names from outer scope
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
          x + y;
        };
        
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        ";

        struct ExpectedToken {
            expected_token_type: TokenType,
            expected_literal: String,
        }

        // A vector of tokens
        let mut tests: Vec<ExpectedToken> = Vec::new();
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Let,
            expected_literal: "let".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "five".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Assign,
            expected_literal: "=".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "5".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Let,
            expected_literal: "let".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "ten".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Assign,
            expected_literal: "=".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "10".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Let,
            expected_literal: "let".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "add".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Assign,
            expected_literal: "=".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Function,
            expected_literal: "fn".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Lparen,
            expected_literal: "(".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "x".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Comma,
            expected_literal: ",".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "y".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Rparen,
            expected_literal: ")".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Lbrace,
            expected_literal: "{".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "x".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Plus,
            expected_literal: "+".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "y".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Rbrace,
            expected_literal: "}".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Let,
            expected_literal: "let".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "result".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Assign,
            expected_literal: "=".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "add".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Lparen,
            expected_literal: "(".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "five".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Comma,
            expected_literal: ",".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Ident,
            expected_literal: "ten".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Rparen,
            expected_literal: ")".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Bang,
            expected_literal: "!".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Minus,
            expected_literal: "-".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Slash,
            expected_literal: "/".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Asterisk,
            expected_literal: "*".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "5".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "5".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Lt,
            expected_literal: "<".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "10".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Gt,
            expected_literal: ">".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "5".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::If,
            expected_literal: "if".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Lparen,
            expected_literal: "(".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "5".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Lt,
            expected_literal: "<".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "10".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Rparen,
            expected_literal: ")".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Lbrace,
            expected_literal: "{".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Return,
            expected_literal: "return".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::True,
            expected_literal: "true".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Rbrace,
            expected_literal: "}".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Else,
            expected_literal: "else".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Lbrace,
            expected_literal: "{".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Return,
            expected_literal: "return".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::False,
            expected_literal: "false".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Rbrace,
            expected_literal: "}".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "10".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Eq,
            expected_literal: "==".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "10".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "10".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::NotEq,
            expected_literal: "!=".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Int,
            expected_literal: "9".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Semicolon,
            expected_literal: ";".to_owned(),
        });
        tests.push(ExpectedToken {
            expected_token_type: TokenType::Eof,
            expected_literal: "".to_owned(),
        });

        let mut l = Lexer::new(input.to_owned());

        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(
                tok.token_type, tt.expected_token_type,
                "tests[{:?}] - tokentype wrong. expected={:?}, got={:?}",
                i, tt.expected_token_type, tok.token_type
            );
            if tok.token_type == tt.expected_token_type {
                println!(
                    "Test type {:?} passed, {:?} == {:?}",
                    i, tt.expected_token_type, tok.token_type
                );
            }
            assert_eq!(
                tok.literal, tt.expected_literal,
                "tests[{:?}] - literal wrong. expected={:?}, got={:?}",
                i, tt.expected_literal, tok.literal
            );
            if tok.literal == tt.expected_literal {
                println!(
                    "Test literal {:?} passed, {:?} == {:?}",
                    i, tt.expected_literal, tok.literal
                );
            }
        }
    }
}

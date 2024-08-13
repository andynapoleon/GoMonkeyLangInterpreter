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

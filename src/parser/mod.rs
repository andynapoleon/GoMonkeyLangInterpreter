use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

pub struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer) -> Self {
        let mut p = Parser {
            l,
            cur_token: None,
            peek_token: None,
        };
        // Read two tokens, so cur_token and peek_token are both set
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, Some(self.l.next_token()));
    }

    // Constructing the root node of the AST
    pub fn parse_program(&mut self) -> Program {
        let program = Program {
            statements: Vec::new(),
        };
        while let Some(cur_token) = self.cur_token {
            if cur_token.token_type != TokenType::Eof {
                let stmt = self.parse_statement();
                match stmt {
                    Some(value) => program.statements.add(value),
                    None => (),
                }
                self.next_token();
            } else {
                break;
            }
        }
        program
    }

    // Parse a statement
    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            Some(TokenType::Let) => self.parse_let_statement(),
            Some(_) => (),
            None => (),
        }
    }

    // Parse a let statement
    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let stmt = LetStatement {
            token: self.cur_token.unwrap(),
            name: None,
            value: None,
        };

        if !self.expect_peek(TokenType::Ident) {
            None
        }

        if let Some(cur_token) = self.cur_token {
            stmt.name = Identifier {
                token: cur_token,
                value: cur_token.literal,
            }
        }

        if !self.expect_peek(TokenType::Assign) {
            None
        }

        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    pub fn cur_token_is(&mut self, t: TokenType) -> bool {
        if let Some(cur_token) = self.cur_token {
            cur_token.token_type == t
        }
    }

    pub fn peek_token_is(&mut self, t: TokenType) -> bool {
        if let Some(peek_token) = self.peek_token {
            peek_token.token_type == t
        }
    }

    pub fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

// Declare modules
#[cfg(test)]
mod tests;

use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

pub struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer) -> Self {
        let mut p = Parser {
            l,
            cur_token: Default::default(),
            peek_token: Default::default(),
        };
        // Read two tokens, so cur_token and peek_token are both set
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.l.next_token());
    }

    // Constructing the root node of the AST
    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        Some(program)
    }

    // Parse a statement
    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => self
                .parse_let_statement()
                .map(|stmt| Box::new(stmt) as Box<dyn Statement>),
            _ => None,
        }
    }

    // Parse a let statement
    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            name: Box::new(Identifier::default()),
            value: Default::default(),
        };

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        stmt.name = Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    pub fn cur_token_is(&mut self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    pub fn peek_token_is(&mut self, t: TokenType) -> bool {
        self.peek_token.token_type == t
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

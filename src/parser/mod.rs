use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

pub struct Parser {
    l: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    fn new(l: Lexer) -> Self {
        let mut p = Self {
            l: l,
            cur_token: None,
            peek_token: None,
        };
        // Read two tokens, so the two pointers get set
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token;
        self.peek_token = Some(self.l.next_token());
    }

    fn parse_program() -> Option<Program> {
        let program = Program {
            statements: Vec::new(),
        };
        None
    }
}

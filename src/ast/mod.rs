use crate::token::*;

// Traits for Node (Statement or Expression) in the AST
pub trait Node {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

// Holds an array of statements - implements Node which represents the ROOT node
pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

pub impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
}

// Identifier - implements an expression node
pub struct Identifier {
    token: Token,
    value: String,
}

pub impl Expression for Identifier {
    fn expression_node(&self) {}
}

pub impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

// Let Statement - implements Statement node
pub struct LetStatement {
    token: Token,
    name: Box<Identifier>,
    value: Box<dyn Expression>,
}

pub impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

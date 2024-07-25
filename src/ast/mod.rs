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
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            static EMPTY: &str = "";
            EMPTY
        }
    }
}

// Default Expression Trait implementation
pub struct DefaultExpression;

impl Expression for DefaultExpression {
    fn expression_node(&self) {}
}

impl Node for DefaultExpression {
    fn token_literal(&self) -> &str {
        "default"
    }
}

impl Default for Box<dyn Expression> {
    fn default() -> Self {
        Box::new(DefaultExpression)
    }
}

// Identifier - implements Expression
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            token: Token::default(),
            value: String::new(),
        }
    }
}

// Let Statement - implements Statement
pub struct LetStatement {
    pub token: Token,
    pub name: Box<Identifier>,
    pub value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Default for LetStatement {
    fn default() -> Self {
        Self {
            token: Token::default(),
            name: Box::new(Identifier::default()),
            value: Default::default(),
        }
    }
}

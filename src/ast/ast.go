package ast

import "github.com/andynapoleon/GoMonkeyLangInterpreter/token"

// Node interface
type Node interface {
	TokenLiteral() string
}

// Statement interface - extends Node
type Statement interface {
	Node
	StatementNode()
}

// Expression interface - extends Node
type Expression interface {
	Node
	expressionNode()
}

// Program structure
type Program struct {
	Statements []Statement // a slice of statements
}

func (p *Program) TokenLiteral() string {
	if len(p.Statements) > 0 {
		return p.Statements[0].TokenLiteral() // token literal of first node
	} else {
		return ""
	}
}

// LetStatement structure - implements Statement
type LetStatement struct {
	Token token.Token // the token.LET token
	Name  *Identifier // example: let x = 5 * 5
	Value Expression  // x is Name (identifier), 5 * 5 is Value (expression)
}

func (ls *LetStatement) StatementNode() {}
func (ls *LetStatement) TokenLiteral() string {
	return ls.Token.Literal
}

// Identifier structure - implements Expression
type Identifier struct {
	Token token.Token // the token.IDENT token
	Value string
}

func (i *Identifier) ExpressionNode() {}
func (i *Identifier) TokenLiteral() string {
	return i.Token.Literal
}

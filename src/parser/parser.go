package parser

import (
	"github.com/andynapoleon/GoMonkeyLangInterpreter/ast"
	"github.com/andynapoleon/GoMonkeyLangInterpreter/lexer"
	"github.com/andynapoleon/GoMonkeyLangInterpreter/token"
)

// Parser structure
type Parser struct {
	l         *lexer.Lexer
	curToken  token.Token
	peekToken token.Token
}

// Create a new Parser structure
func New(l *lexer.Lexer) *Parser { // takes in a lexer
	p := &Parser{l: l} // curToken and peekToken are 0
	// Read two tokens, so curToken and peekToken are both set
	p.nextToken()
	p.nextToken()
	return p
}

// Move to next token in the lexer
func (p *Parser) nextToken() {
	p.curToken = p.peekToken
	p.peekToken = p.l.NextToken()
}

// Parse a program
func (p *Parser) ParseProgram() *ast.Program {
	program := &ast.Program{}
	program.Statements = []ast.Statement{}

	for p.curToken.Type != token.EOF {
		stmt := p.parseStatement()
		if stmt != nil {
			program.Statements = append(program.Statements, stmt)
		}
		p.nextToken()
	}
	return program
}

// Parse a statement
func (p *Parser) parseStatement() ast.Statement {
	switch p.curToken.Type {
	case token.LET:
		return p.parseLetStatement()
	default:
		return nil
	}

}

// Parse a LET statement
func (p *Parser) parseLetStatement() *ast.LetStatement {
	return nil
}

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

func New(l *lexer.Lexer) *Parser {
	p := &Parser{l: l} // curToken and peekToken are 0
	// Read two tokens, so curToken and peekToken are both set
	p.nextToken()
	p.nextToken()
	return p
}

func (p *Parser) nextToken() {
	p.curToken = p.peekToken
	p.peekToken = p.l.NextToken()
}

func (p Parser) ParseProgram() *ast.Program {
	return nil
}

use super::*;

#[test]
fn test_let_statements() {
    let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    "
    .to_string();

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser
        .parse_program()
        .expect("ParseProgram() returned None");

    assert_eq!(
        program.statements.len(),
        3,
        "program.Statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    let tests = vec!["x", "y", "foobar"];

    for (i, expected_identifier) in tests.iter().enumerate() {
        let stmt = &program.statements[i];
        assert!(test_let_statement(stmt, expected_identifier));
    }
}

fn test_let_statement(s: &Statement, name: &str) -> bool {
    if s.token_literal() != "let" {
        panic!("s.TokenLiteral not 'let'. got={}", s.token_literal());
    }

    if let Statement::Let(let_stmt) = s {
        if let_stmt.name.value != name {
            panic!(
                "letStmt.Name.Value not '{}'. got={}",
                name, let_stmt.name.value
            );
        }

        if let_stmt.name.token_literal() != name {
            panic!(
                "letStmt.Name.TokenLiteral not '{}'. got={}",
                name,
                let_stmt.name.token_literal()
            );
        }

        true
    } else {
        panic!("s not LetStatement. got={:?}", s);
    }
}

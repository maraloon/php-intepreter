use crate::{
    ast::ast::{self, Statement},
    lexer::lexer::{Lexer, Token, Value},
};

pub enum Symbol {
    Lowest,
    Equals,      // ==
    Lessgreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(x)
    Index,       // array[index]
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let first = lexer.next_token().unwrap();
        let second = lexer.next_token().unwrap();

        return Parser {
            lexer,
            current_token: first,
            peek_token: second,
            errors: vec![],
        };
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut statements: Vec<Statement> = vec![];

        loop {
            if self.current_token == Token::Eof {
                break;
            }
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(e) => print!("Parse error: {}", e),
            }
            self.next_token();
        }

        return ast::Program { statements };
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().unwrap();
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, String> {
        match &self.current_token {
            Token::Var(v) => self.parse_var_statement(v.clone()),
            // todo
            _ => Ok(ast::Statement::Var(ast::VarStatement {
                token: Token::Var(Value::Empty),
                name: "x".to_string(),
                value: "7".to_string(),
            })),
        }
    }

    fn parse_var_statement(&mut self, var_name: Value) -> Result<ast::Statement, String> {
        self.expect_peek(Token::Assign)?;
        self.next_token();
        // todo
        // let value = self.parse_expression();
        let value = "5".to_string();

        if self.peek_token == Token::Semicolon {
            self.next_token()
        }

        Ok(ast::Statement::Var(ast::VarStatement {
            token: Token::Var(Value::Empty),
            name: var_name.to_string(),
            value,
        }))
    }

    fn expect_peek(&mut self, token: Token) -> Result<(), String> {
        if self.peek_token == token {
            self.next_token();
            Ok(())
        } else {
            Err(format!(
                "expected next token to be {}, got {} instead",
                token, self.peek_token
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::ast::{self, Node},
        lexer::lexer::{Token, Value},
    };

    use super::{Lexer, Parser};

    #[test]
    fn var_statement() {
        let input = ("$x = 5;", "x", "5");

        let lexer = Lexer::new(String::from(input.0));
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(parser);

        if program.statements.len() != 1 {
            panic!(
                "program.statements does not contain 1 statements. got={}",
                program.statements.len()
            )
        }

        let statement = &program.statements[0];
        test_var_statement(statement, input.1, input.2);
    }

    fn test_var_statement(var_statement: &ast::Statement, name: &str, value: &str) {
        match var_statement {
            ast::Statement::Var(var_statement) => {
                // todo а нахуя в итоге это поле и проверка нужна?
                if var_statement.token() != Token::Var(Value::Empty) {
                    panic!(
                        "statement.token_literal not {}. got={}",
                        Token::Var(Value::Empty),
                        var_statement.token()
                    )
                }

                if var_statement.name != name {
                    panic!(
                        "var_statement.name not '{}'. got={}",
                        name, var_statement.name
                    )
                }
                // todo expression check
                if var_statement.value != value {
                    panic!("statement.value not {}, got {}", value, var_statement.value);
                }
            }
            _ => panic!("expected ast::Statement::Var(var_statement)"),
        }
    }

    fn check_parser_errors(parser: Parser) {
        let errors = parser.errors;

        if errors.len() == 0 {
            return;
        }

        print!("Parser has {} errors", errors.len());
        for msg in errors {
            print!("parse error: {}", msg);
        }
        panic!();
    }
}

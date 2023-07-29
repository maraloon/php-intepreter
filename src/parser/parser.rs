use crate::{
    ast::ast::{self, Statement, VarStatement},
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

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match &self.current_token {
            Token::Var(v) => self.parse_var_statement(v.clone()),
            Token::Return => self.parse_return_statement(),
            // todo
            _ => panic!("parse_statement()"),
        }
    }

    fn parse_expression(&mut self) -> String {
        match &self.current_token {
            Token::Int(v) => v.clone().to_string(),
            _ => todo!(),
        }
    }

    fn parse_var_statement(&mut self, var_name: Value) -> Result<Statement, String> {
        self.expect_peek(Token::Assign)?;
        self.next_token();
        let value = self.parse_expression();

        if self.peek_token == Token::Semicolon {
            self.next_token()
        }

        Ok(Statement::Var(VarStatement {
            name: var_name.to_string(),
            value,
        }))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.next_token();
        let value = self.parse_expression();
        self.next_token();

        if self.peek_token == Token::Semicolon {
            self.next_token()
        }

        Ok(Statement::Return(ast::ReturnStatement {
            return_value: value,
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
    use crate::ast::ast::{
        self,
        Statement::{Return, Var},
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

    #[test]
    fn return_statement() {
        // let input = ("return 5;", 5);
        let input = ("return 5;", "5");

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
        match statement {
            Return(return_statement) => {
                if return_statement.return_value != input.1 {
                    panic!(
                        "return_statement.return_value not '{}'. got={}",
                        input.1, return_statement.return_value
                    )
                }
            }
            _ => panic!("expected ast::Statement::Return()"),
        }
    }

    fn test_var_statement(var_statement: &ast::Statement, name: &str, value: &str) {
        match var_statement {
            Var(var_statement) => {
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
            _ => panic!("expected ast::Statement::Var()"),
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

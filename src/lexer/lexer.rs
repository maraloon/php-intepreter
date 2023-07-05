use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Empty,
    Is(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Is(value) => write!(f, "{}", value),
            Value::Empty => write!(f, ""),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal(String),
    Eof,
    Ident(Value), // add, foobar, x, y, ...
    Int(Value),
    Var(Value),
    Assign, // =
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    LT,
    // GT,
    Function,
    True,
    False,
    If,
    Else,
    Return,
    StartTag, // <?php
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Ident(x) => write!(f, "Ident({})", x),
            Token::Var(x) => write!(f, "Var({})", x),
            Token::Int(x) => write!(f, "Int({})", x),
            Token::Illegal(e) => write!(f, "Illegal({})", e),
            Token::Eof => write!(f, "Eof"),
            Token::Assign => write!(f, "Assign"),
            Token::Plus => write!(f, "Plus"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Lparen => write!(f, "Lparen"),
            Token::Rparen => write!(f, "Rparen"),
            Token::Lbrace => write!(f, "Lbrace"),
            Token::Rbrace => write!(f, "Rbrace"),
            Token::LT => write!(f, "LT"),
            // Token::GT => write!(f, "GT"),
            Token::Function => write!(f, "Function"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::Return => write!(f, "Return"),
            Token::StartTag => write!(f, "StartTag"),
        };
    }
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    fn skip_whitespaces(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespaces();

        let token = match self.ch {
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'<' => {
                if self.peek() == b'?' {
                    self.read_char();
                    self.read_char();
                    let literal = self.read_identifier();
                    return Ok(match literal.as_str() {
                        "php" => Token::StartTag,
                        _ => Token::Illegal(String::from("False symbols after <?")),
                    });
                } else {
                    Token::LT
                }
            }
            b'$' => {
                self.read_char();
                let var_name = self.read_var_name();
                return Ok(match var_name {
                    Ok(_) => Token::Var(Value::Is(var_name.unwrap())),
                    Err(e) => Token::Illegal(e.to_string()),
                });
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let literal = self.read_identifier();
                return Ok(match literal.as_str() {
                    "function" => Token::Function,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Ident(Value::Is(literal)),
                });
            }
            b'0'..=b'9' => {
                return Ok(Token::Int(Value::Is(self.read_int())));
            }
            0 => Token::Eof,
            _ => unreachable!("if you see it - shame on you"),
        };

        self.read_char();
        return Ok(token);
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    fn read_int(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[position..self.position]).to_string();
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[position..self.position]).to_string();
    }

    fn read_var_name(&mut self) -> Result<String, &str> {
        let position = self.position;

        if self.ch.is_ascii_digit() {
            return Err("First symbol of var can't be digit");
        }

        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }

        return Ok(String::from_utf8_lossy(&self.input[position..self.position]).to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token, Value};

    #[test]
    fn get_next_token() -> Result<(), String> {
        let input = "<?php
        $five = 5;
        $ten = 10;

        $add = function($x, $y) {
            return $x + $y;
        };

        $result = add($five, $ten);";

        let tokens = vec![
            Token::StartTag,
            Token::Var(Value::Is(String::from("five"))),
            Token::Assign,
            Token::Int(Value::Is(String::from("5"))),
            Token::Semicolon,
            Token::Var(Value::Is(String::from("ten"))),
            Token::Assign,
            Token::Int(Value::Is(String::from("10"))),
            Token::Semicolon,
            Token::Var(Value::Is(String::from("add"))),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Var(Value::Is(String::from("x"))),
            Token::Comma,
            Token::Var(Value::Is(String::from("y"))),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::Var(Value::Is(String::from("x"))),
            Token::Plus,
            Token::Var(Value::Is(String::from("y"))),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Var(Value::Is(String::from("result"))),
            Token::Assign,
            Token::Ident(Value::Is(String::from("add"))),
            Token::Lparen,
            Token::Var(Value::Is(String::from("five"))),
            Token::Comma,
            Token::Var(Value::Is(String::from("ten"))),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.into());
        for expected_token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", expected_token, next_token);
            assert_eq!(expected_token, next_token);
        }

        return Ok(());
    }

    #[test]
    fn false_var_name() -> Result<(), String> {
        let input = "$1five";

        let tokens = vec![Token::Illegal(String::from(
            "First symbol of var can't be digit",
        ))];

        let mut lexer = Lexer::new(input.into());
        for expected_token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", expected_token, next_token);
            assert_eq!(expected_token, next_token);
        }

        return Ok(());
    }
}

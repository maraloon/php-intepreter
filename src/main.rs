pub mod lexer;
pub mod ast;
pub mod parser;

use std::io;

use users::get_current_username;

use crate::lexer::lexer::{Lexer, Token};

fn main() -> ! {
    match get_current_username() {
        Some(uname) => println!("Running php REPL as {:?}", uname),
        None => println!("The current user does not exist!"),
    }

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let mut lexer = Lexer::new(input.clone());

                loop {
                    let token = lexer.next_token().unwrap();
                    if token == Token::Eof {
                        break;
                    }
                    println!("{}", token);
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

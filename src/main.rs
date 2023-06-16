// todo delete this line
#![allow(unused)]

pub mod lexer;

use std::io;

use users::get_current_username;

fn main() {
    match get_current_username() {
        Some(uname) => println!("Running php REPL as {:?}", uname),
        None => println!("The current user does not exist!"),
    }

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer);
        println!("{}", buffer);
    }
}

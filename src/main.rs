use crate::lexer::lexer::lex;
use crate::parser::parser::parse;
use std::io;
use std::borrow::BorrowMut;

mod lexer;
mod parser;

fn main() {
    run_repl();
}

pub fn run_repl() {
    loop {
        let line = read_line();
        let mut tokens = lex(&line).unwrap();
        let ast = parse(&mut tokens).unwrap();
        println!("{:?}", ast);
    }
}

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            trim_newline(input.borrow_mut());
            input
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

use std::io;
use std::borrow::BorrowMut;
use crate::vm::vm::interpret;

mod lexer;
mod parser;
mod compiler;
mod vm;

fn main() {
    run_repl();
}

pub fn run_repl() {
    loop {
        interpret(&read_line());
    }
}

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
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

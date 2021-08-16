use crate::lexer::lexer::lex;
use crate::parser::parser::parse;
use std::io;
use std::borrow::BorrowMut;
use crate::hm::syntax::Syntax::Lambda;
use crate::hm::syntax::*;
use crate::hm::infer::analyse;
use std::collections::{HashSet, HashMap};
use crate::hm::env::Env;
use crate::hm::types::TypeVarGenerator;

mod lexer;
mod parser;
mod hm;

fn main() {
    let (mut a, env) = Env::new();

    let syntax = let_("f", lambda("x", ident("x")), ident("5"));

    let t = analyse(&syntax, &mut a, &env, &HashSet::new());

    println!("{}", (a[t].as_string(&a, &mut TypeVarGenerator::new())));

    // run_repl();
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

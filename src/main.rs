use crate::lexer::lexer::lex;
use crate::parser::parser::parse;

mod lexer;
mod parser;

fn main() {
    let mut tokens = lex("let x = 10;").unwrap();
    let ast = parse(&mut tokens);
    println!("foobar");
}

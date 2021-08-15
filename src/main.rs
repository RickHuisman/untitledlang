use crate::lexer::lexer::lex;

mod lexer;

fn main() {
    let tokens = lex("lex x = 10;").unwrap();
    println!("{:?}", tokens);
}

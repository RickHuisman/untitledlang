use crate::lexer::error::LexResult;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

pub mod error;
mod lexer;
pub mod token;

pub fn lex(source: &str) -> LexResult<Vec<Token>> {
    let mut lexer = Lexer::new(source);

    let mut tokens = vec![];
    // TODO: Loop?
    loop {
        if let Some(token) = lexer.read_token()? {
            if let TokenType::EOF = token.token_type() {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
    }

    Ok(tokens)
}

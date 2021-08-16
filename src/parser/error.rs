use crate::lexer::token::TokenType;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,
    ExpectedPrimary(TokenType),
    Expect(TokenType, TokenType, usize),
}

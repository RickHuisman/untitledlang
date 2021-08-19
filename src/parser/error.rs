use crate::lexer::token::TokenType;

pub type ParseResult<T> = std::result::Result<T, ParserError>;

// TODO: Use Token not TokenType.
#[derive(Debug)]
pub enum ParserError {
    Expected(TokenType, TokenType, usize),
    Unexpected(TokenType),
    ExpectedPrimary(TokenType),
    ExpectedUnaryOperator(TokenType),
    ExpectedBinaryOperator(TokenType),
    UnexpectedEOF,
}

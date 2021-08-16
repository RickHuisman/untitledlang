use crate::lexer::token::TokenType;
use crate::parser::error::ParserError;
use crate::parser::ast::*;
use crate::parser::parser::Parser;

type Result<T> = std::result::Result<T, ParserError>;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assign,
    // =
    Or,
    And,
    Equality,
    // == !=
    Comparison,
    // < <= > >=
    Term,
    // + -
    Factor,
    // * /
    Unary,
    // ! -
    Call,
    // ()
    Primary,
}

impl<'a> From<&TokenType> for Precedence {
    fn from(token: &TokenType) -> Precedence {
        match token {
            TokenType::Equal => Precedence::Assign,
            TokenType::BangEqual | TokenType::EqualEqual => Precedence::Equality,
            TokenType::LessThan
            | TokenType::LessThanEqual
            | TokenType::GreaterThan
            | TokenType::GreaterThanEqual => Precedence::Comparison,
            TokenType::Plus | TokenType::Minus => Precedence::Term,
            TokenType::Star | TokenType::Slash => Precedence::Factor,
            TokenType::Bang => Precedence::Unary,
            TokenType::LeftParen => Precedence::Call,
            TokenType::Dot => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

pub fn parse(parser: &mut Parser) -> Result<Expr> {
    parse_expr(parser, Precedence::None)
}

fn parse_expr(parser: &mut Parser, precedence: Precedence) -> Result<Expr> {
    let mut expr = parse_prefix(parser)?;
    while !parser.is_eof()? {
        let next_precedence = Precedence::from(parser.peek_type()?);
        if precedence >= next_precedence {
            break;
        }
        expr = parse_infix(parser, expr)?;
    }
    Ok(expr)
}

fn parse_prefix(parser: &mut Parser) -> Result<Expr> {
    match parser.peek_type()? {
        TokenType::Number
        | TokenType::Identifier
        | TokenType::String => parse_primary(parser),
        TokenType::Bang | TokenType::Minus => parse_unary(parser),
        TokenType::LeftParen => parse_grouping(parser),
        _ => todo!(),
        // _ => Err(SyntaxError::Unexpected(parser.peek_token().clone())), TODO
    }
}

fn parse_infix(parser: &mut Parser, left: Expr) -> Result<Expr> {
    match parser.peek_type()? {
        TokenType::BangEqual
        | TokenType::EqualEqual
        | TokenType::LessThan
        | TokenType::LessThanEqual
        | TokenType::GreaterThan
        | TokenType::GreaterThanEqual
        | TokenType::Plus
        | TokenType::Minus
        | TokenType::Star
        | TokenType::Slash => parse_binary(parser, left),
        _ => todo!(),
        // _ => Err(SyntaxError::Unexpected(parser.peek_token().clone())),
    }
}

fn parse_primary(parser: &mut Parser) -> Result<Expr> {
    let token = parser.consume()?;
    match token.token_type() {
        TokenType::Number => Ok(Expr::Literal(LiteralExpr::Number(
            token.source().parse::<f64>().unwrap(),
        ))),
        TokenType::Identifier => {
            let ident = token.source().to_string();

            Ok(if parser.match_(&TokenType::Equal)? {
                let expr = parser.expression()?;
                Expr::let_set(ident, expr)
            } else {
                Expr::let_get(ident)
            })
        }
        _ => Err(ParserError::ExpectedPrimary(token.token_type().clone())),
    }
}

fn parse_binary(parser: &mut Parser, left: Expr) -> Result<Expr> {
    let precedence = Precedence::from(parser.peek_type()?);
    let operator = BinaryOperator::from_token(
        parser.consume()?.token_type()
    ).unwrap(); // TODO Unwrap
    let right = parse_expr(parser, precedence)?;

    Ok(Expr::binary(left, operator, right))
}

fn parse_unary(parser: &mut Parser) -> Result<Expr> {
    let operator = UnaryOperator::from_token(
        parser.consume()?.token_type()
    ).unwrap(); // TODO Unwrap
    let right = parse_expr(parser, Precedence::Unary)?;

    Ok(Expr::unary(operator, right))
}

fn parse_grouping(parser: &mut Parser) -> Result<Expr> {
    parser.expect(TokenType::LeftParen)?;
    let expr = parse_expr(parser, Precedence::None)?;
    parser.expect(TokenType::RightParen)?;

    Ok(Expr::grouping(expr))
}

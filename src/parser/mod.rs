use crate::lexer::token::Token;
use crate::parser::ast::ModuleAst;
use crate::parser::error::ParserError;
use crate::parser::parser::Parser;

pub mod ast;
mod error;
mod expr_parser;
mod parser;

type ParserResult<'a, T> = std::result::Result<T, ParserError>;

pub fn parse<'a>(tokens: &'a mut Vec<Token<'a>>) -> ParserResult<ModuleAst> {
    let mut parser = Parser::new(tokens);

    let mut ast = vec![];
    while !parser.is_eof()? {
        ast.push(parser.parse_top_level_expr()?);
    }

    Ok(ast)
}

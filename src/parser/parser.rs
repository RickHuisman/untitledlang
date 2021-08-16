use crate::lexer::token::{TokenType, Token, Keyword};
use crate::parser::ast::*;
use crate::parser::error::ParserError;
use crate::parser::expr_parser;

type Result<T> = std::result::Result<T, ParserError>;

pub fn parse<'a>(tokens: &'a mut Vec<Token<'a>>) -> Result<ModuleAst> {
    let mut parser = Parser::new(tokens);

    let mut ast = vec![];
    while !(parser.is_eof()?) {
        ast.push(parser.parse_top_level_expr()?);
    }

    Ok(ast)
}

pub struct Parser<'a> {
    tokens: &'a mut Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a mut Vec<Token<'a>>) -> Self {
        tokens.reverse();
        Parser { tokens }
    }

    fn parse_top_level_expr(&mut self) -> Result<Expr> {
        match self.peek_type()? {
            TokenType::Keyword(Keyword::Let) => self.declare_let(),
            _ => self.parse_expression_statement(),
        }
    }

    fn declare_let(&mut self) -> Result<Expr> {
        // Consume "let".
        self.expect(TokenType::Keyword(Keyword::Let))?;

        let ident = self.parse_ident()?;

        let initializer = if self.match_(&TokenType::Equal)? {
            self.parse_expression_statement()?
        } else {
            self.expect(TokenType::Semicolon)?;
            Expr::Literal(LiteralExpr::Nil)
        };

        Ok(Expr::LetAssign(LetAssignExpr::new(
            ident,
            Box::new(initializer),
        )))
    }

    fn parse_block(&mut self) -> Result<BlockExpr> {
        // Consume '{'.
        self.expect(TokenType::LeftBrace);

        let mut exprs = vec![];
        while !self.match_(&TokenType::RightBrace)? {
            exprs.push(self.parse_top_level_expr()?);
        }

        Ok(BlockExpr::new(exprs))
    }

    pub fn parse_expression_statement(&mut self) -> Result<Expr> {
        let expr = self.expression()?;
        self.expect(TokenType::Line)?;
        Ok(expr)
    }

    pub fn parse_ident(&mut self) -> Result<Identifier> {
        Ok(self.expect(TokenType::Identifier)?.source().to_string())
    }

    pub fn expression(&mut self) -> Result<Expr> {
        expr_parser::parse(self)
    }

    pub fn expect(&mut self, expect: TokenType) -> Result<Token<'a>> {
        if self.check(&expect)? {
            Ok(self.consume()?)
        } else {
            Err(ParserError::Expect(
                expect,
                self.peek_type()?.clone(), // TODO Clone
                self.peek().unwrap().position().line().clone(), // TODO Clone
            ))
        }
    }

    pub fn match_(&mut self, token_type: &TokenType) -> Result<bool> {
        if !self.check(token_type)? {
            return Ok(false);
        }

        self.consume()?;
        Ok(true)
    }

    fn check(&self, token_type: &TokenType) -> Result<bool> {
        Ok(self.peek_type()? == token_type)
    }

    pub fn consume(&mut self) -> Result<Token<'a>> {
        self.tokens
            .pop()
            .ok_or(ParserError::UnexpectedEOF)
    }

    pub fn peek_type(&self) -> Result<&TokenType> {
        Ok(self.peek()?.token_type())
    }

    fn peek(&self) -> Result<&Token<'a>> {
        self.tokens
            .last()
            .ok_or(ParserError::UnexpectedEOF)
    }

    pub fn is_eof(&self) -> Result<bool> {
        Ok(self.check(&TokenType::EOF)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer::lex;

    fn run_test(expect: Vec<Expr>, source: &str) {
        let mut tokens = lex(source).unwrap();
        let actual = parse(&mut tokens).unwrap();

        assert_eq!(expect, actual)
    }

    #[test]
    fn parse_assign_let() {
        let expect = vec![Expr::LetAssign(LetAssignExpr::new(
            "x".to_string(),
            Box::new(Expr::Literal(LiteralExpr::Number(5.0))),
        ))];

        let source = "let x = 5;";
        run_test(expect, source);
    }

    #[test]
    fn parse_set_let() {
        let expect = vec![Expr::LetSet(LetSetExpr::new(
            "x".to_string(),
            Box::new(Expr::Literal(LiteralExpr::Number(5.0))),
        ))];

        let source = "x = 5;";
        run_test(expect, source);
    }

    #[test]
    fn parse_get_let() {
        let expect = vec![
            Expr::LetAssign(LetAssignExpr::new(
                "x".to_string(),
                Box::new(Expr::Literal(LiteralExpr::Number(5.0))),
            )),
            Expr::LetAssign(LetAssignExpr::new(
                "y".to_string(),
                Box::new(Expr::LetGet(LetGetExpr::new(
                    "x".to_string(),
                ))),
            )),
        ];

        let source = "let x = 5; let y = x;";
        run_test(expect, source);
    }

    #[test]
    fn parse_block() {
        let expect = vec![
            Expr::Block(
                BlockExpr::new(vec![
                    Expr::LetAssign(LetAssignExpr::new(
                        "x".to_string(),
                        Box::new(Expr::Literal(LiteralExpr::Number(5.0))),
                    )),
                    Expr::LetAssign(LetAssignExpr::new(
                        "y".to_string(),
                        Box::new(Expr::LetGet(LetGetExpr::new(
                            "x".to_string(),
                        ))),
                    )),
                ])
            )
        ];

        let source = r#"
        {
            let x = 5;
            let y = x;
        }
        "#;
        run_test(expect, source);
    }

    #[test]
    fn parse_grouping() {
        let expect = vec![
            Expr::Grouping(
                GroupingExpr::new(Box::new(Expr::Binary(BinaryExpr::new(
                    Box::new(Expr::Literal(LiteralExpr::Number(2.0))),
                    BinaryOperator::Add,
                    Box::new(Expr::Literal(LiteralExpr::Number(4.0))),
                )))),
            ),
        ];

        let source = r#"(2 + 4);"#;
        run_test(expect, source);
    }
}

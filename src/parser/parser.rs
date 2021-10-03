use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::*;
use crate::parser::error::{ParseResult, ParserError};
use crate::parser::expr_parser;

pub struct Parser<'a> {
    tokens: &'a mut Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Vec<Token<'a>>) -> Self {
        tokens.reverse();
        Parser { tokens }
    }

    pub fn parse_top_level_expr(&mut self) -> ParseResult<Expr> {
        match self.peek_type()? {
            TokenType::Let => self.parse_let(),
            TokenType::Fun => self.parse_fun(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::If => self.parse_if(),
            TokenType::Print => self.parse_print(),
            TokenType::LeftBrace => self.parse_block(),
            TokenType::Return => self.parse_return(),
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_let(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Let)?;

        let ident = self.parse_ident()?;

        let initializer = if self.match_(TokenType::Equal)? {
            self.parse_expr_statement()?
        } else {
            self.expect(TokenType::Semicolon)?;
            Expr::Literal(LiteralExpr::Nil)
        };

        Ok(Expr::let_assign(ident, initializer))
    }

    fn parse_fun(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Fun)?;

        let ident = self.parse_ident()?;
        let args = self.parse_args()?;

        let body = self.block()?;
        let fun_decl = FunDecl::new(args, body);

        Ok(Expr::fun(ident, fun_decl))
    }

    fn parse_while(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::While)?;

        let cond = self.expression()?;
        let body = self.parse_block()?;

        Ok(Expr::while_(cond, body))
    }

    fn parse_for(&mut self) -> ParseResult<Expr> {
        todo!()
    }

    fn parse_if(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::If)?;

        let cond = self.expression()?;

        // Then branch.
        let then = self.block()?;

        // Else branch.
        let else_clause = if self.match_(TokenType::Else)? {
            Some(self.block()?)
        } else {
            None
        };

        Ok(Expr::if_else(cond, then, else_clause))
    }

    fn parse_print(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Print)?;
        let expr = self.parse_expr_statement()?;
        Ok(Expr::print(expr))
    }

    fn parse_block(&mut self) -> ParseResult<Expr> {
        Ok(Expr::block(self.block()?))
    }

    fn parse_return(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Return)?;

        let expr = if self.match_(TokenType::Semicolon)? {
            // return;
            None
        } else {
            // return <expr>;
            Some(self.parse_top_level_expr()?)
        };

        Ok(Expr::return_(expr))
    }

    pub fn parse_expr_statement(&mut self) -> ParseResult<Expr> {
        let expr = self.expression()?;
        self.match_(TokenType::Semicolon)?;
        Ok(expr)
    }

    pub fn expression(&mut self) -> ParseResult<Expr> {
        expr_parser::parse(self)
    }

    pub fn parse_ident(&mut self) -> ParseResult<Identifier> {
        Ok(self.expect(TokenType::Identifier)?.source().to_string())
    }

    pub fn parse_args(&mut self) -> ParseResult<Vec<Identifier>> {
        self.expect(TokenType::LeftParen)?;

        let mut args = vec![];
        while !self.check(TokenType::RightParen)? && !self.check(TokenType::EOF)? {
            args.push(self.parse_ident()?);

            if !self.match_(TokenType::Comma)? {
                break;
            }
        }

        self.expect(TokenType::RightParen)?;

        Ok(args)
    }

    fn block(&mut self) -> ParseResult<BlockDecl> {
        self.expect(TokenType::LeftBrace);

        let mut exprs = vec![];
        while !self.match_(TokenType::RightBrace)? {
            exprs.push(self.parse_top_level_expr()?);
        }

        Ok(exprs)
    }

    pub fn expect(&mut self, expect: TokenType) -> ParseResult<Token<'a>> {
        if self.check(expect.clone())? {
            // TODO: Clone
            return Ok(self.consume()?);
        }

        Err(ParserError::Expected(
            expect.clone(),                         // TODO: Clone
            self.peek_type()?.clone(),              // TODO: Clone
            self.peek()?.position().line().clone(), // TODO: Clone
        ))
    }

    pub fn consume(&mut self) -> ParseResult<Token<'a>> {
        self.tokens.pop().ok_or(ParserError::UnexpectedEOF)
    }

    pub fn peek(&self) -> ParseResult<&Token<'a>> {
        self.tokens.last().ok_or(ParserError::UnexpectedEOF)
    }

    pub fn peek_type(&self) -> ParseResult<&TokenType> {
        Ok(self.peek()?.token_type())
    }

    pub fn match_(&mut self, token_type: TokenType) -> ParseResult<bool> {
        if !self.check(token_type)? {
            return Ok(false);
        }

        self.consume()?;
        Ok(true)
    }

    pub fn check(&self, token_type: TokenType) -> ParseResult<bool> {
        Ok(self.peek_type()? == &token_type)
    }

    pub fn is_eof(&self) -> ParseResult<bool> {
        Ok(self.check(TokenType::EOF)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parse;

    fn run_test(expect: Vec<Expr>, source: &str) {
        let mut tokens = lex(source).unwrap();
        let actual = parse(&mut tokens).unwrap();

        assert_eq!(expect, actual)
    }

    #[test]
    fn parse_assign_let() {
        let expect = vec![Expr::let_assign(
            "x".to_string(),
            Expr::Literal(LiteralExpr::Number(5.0)),
        )];

        let source = "let x = 5;";
        run_test(expect, source);
    }

    #[test]
    fn parse_set_let() {
        let expect = vec![Expr::let_set(
            "x".to_string(),
            Expr::Literal(LiteralExpr::Number(5.0)),
        )];

        let source = "x = 5;";
        run_test(expect, source);
    }

    #[test]
    fn parse_get_let() {
        let expect = vec![
            Expr::let_assign("x".to_string(), Expr::Literal(LiteralExpr::Number(5.0))),
            Expr::let_assign("y".to_string(), Expr::let_get("x".to_string())),
        ];

        let source = r#"let x = 5; let y = x;"#;
        run_test(expect, source);
    }

    #[test]
    fn parse_while() {
        let expect = vec![Expr::while_(
            Expr::binary(
                Expr::let_get("x".to_string()),
                BinaryOperator::LessThan,
                Expr::Literal(LiteralExpr::Number(5_f64)),
            ),
            Expr::block(vec![Expr::print(Expr::let_get("x".to_string()))]),
        )];

        let source = r#"
        while x < 5 {
            print x;
        }
        "#;
        run_test(expect, source);
    }

    #[test]
    fn parse_block() {
        let expect = vec![Expr::block(vec![
            Expr::let_assign("x".to_string(), Expr::Literal(LiteralExpr::Number(5.0))),
            Expr::let_assign("y".to_string(), Expr::let_get("x".to_string())),
        ])];

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
        let expect = vec![Expr::grouping(Expr::binary(
            Expr::Literal(LiteralExpr::Number(2.0)),
            BinaryOperator::Add,
            Expr::Literal(LiteralExpr::Number(4.0)),
        ))];

        let source = r#"(2 + 4);"#;
        run_test(expect, source);
    }

    #[test]
    fn parse_if() {
        let expect = vec![Expr::if_else(
            Expr::binary(
                Expr::let_get("x".to_string()),
                BinaryOperator::LessThan,
                Expr::Literal(LiteralExpr::Number(5_f64)),
            ),
            vec![Expr::return_(Some(Expr::Literal(LiteralExpr::True)))],
            None,
        )];

        let source = r#"
        if x < 5 {
            return true;
        }
        "#;
        run_test(expect, source);
    }

    #[test]
    fn parse_if_else() {
        let expect = vec![Expr::if_else(
            Expr::binary(
                Expr::let_get("x".to_string()),
                BinaryOperator::LessThan,
                Expr::Literal(LiteralExpr::Number(5_f64)),
            ),
            vec![Expr::return_(Some(Expr::Literal(LiteralExpr::True)))],
            Some(vec![Expr::return_(Some(Expr::Literal(LiteralExpr::False)))]),
        )];

        let source = r#"
        if x < 5 {
            return true;
        }
        else {
            return false;
        }
        "#;
        run_test(expect, source);
    }

    #[test]
    fn parse_return() {
        let expect = vec![Expr::return_(None)];

        let source = "return;";
        run_test(expect, source);
    }

    #[test]
    fn parse_return_value() {
        let expect = vec![Expr::return_(Some(Expr::Literal(LiteralExpr::True)))];

        let source = "return true;";
        run_test(expect, source);
    }

    #[test]
    fn parse_def() {
        let expect = vec![Expr::fun(
            "foo".to_string(),
            FunDecl::new(
                vec![],
                vec![Expr::return_(Some(Expr::Literal(LiteralExpr::True)))],
            ),
        )];

        let source = r#"
        fun foo() {
            return true
        }
        "#;
        run_test(expect, source);
    }
}

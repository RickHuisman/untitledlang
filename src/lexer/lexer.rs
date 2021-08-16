use std::iter::Peekable;
use std::str::{CharIndices, FromStr};
use crate::lexer::token::*;
use crate::lexer::error::SyntaxError;

type Result<T> = std::result::Result<T, SyntaxError>;

pub fn lex(source: &str) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(source);

    let mut tokens = vec![];
    loop {
        let token = lexer.read_token()?;
        if let TokenType::EOF = token.token_type() {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    Ok(tokens)
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.char_indices().peekable(),
            line: 1,
        }
    }

    fn read_token(&mut self) -> Result<Token<'a>> {
        self.skip_whitespace();
        if self.is_at_end() {
            return self.eof();
        }

        let (start, c) = self
            .advance()
            .ok_or(SyntaxError::UnexpectedEOF)?;

        if c.is_alphabetic() {
            return self.identifier(start);
        }
        if c.is_digit(10) {
            return self.number(start);
        }

        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ';' | '\n' | '\r' => {
                self.skip_lines();
                TokenType::Line
            }
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            '!' => {
                if self.check('=')? {
                    self.advance();
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '>' => {
                if self.check('=')? {
                    self.advance();
                    TokenType::GreaterThanEqual
                } else {
                    TokenType::GreaterThan
                }
            }
            '<' => {
                if self.check('=')? {
                    self.advance();
                    TokenType::LessThanEqual
                } else {
                    TokenType::LessThan
                }
            }
            '=' => {
                if self.check('=')? {
                    self.advance();
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '"' => self.string()?,
            _ => todo!(), // TODO: ???
        };

        Ok(self.make_token(token_type, start))
    }

    fn identifier(&mut self, start: usize) -> Result<Token<'a>> {
        self.advance_while(|&c| c.is_alphanumeric());

        let source = self.token_contents(start);

        let token_type = Keyword::from_str(source)
            .map(TokenType::Keyword)
            .unwrap_or(TokenType::Identifier);

        Ok(self.make_token(token_type, start))
    }

    fn number(&mut self, start: usize) -> Result<Token<'a>> {
        self.advance_while(|c| c.is_digit(10));

        // Look for a fractional part
        if let Some(peek) = self.peek() {
            if peek == '.' {
                if let Some(next) = self.peek_next() {
                    if next.is_digit(10) {
                        // Consume the '.'.
                        self.advance();

                        self.advance_while(|c| c.is_digit(10));
                    }
                }
            }
        }

        Ok(self.make_token(TokenType::Number, start))
    }

    fn string(&mut self) -> Result<TokenType> {
        self.advance_while(|&c| c != '"');
        if self.is_at_end() {
            return Err(SyntaxError::UnterminatedString);
        }

        // Consume the '"'.
        self.advance();

        Ok(TokenType::String)
    }

    fn eof(&mut self) -> Result<Token<'a>> {
        Ok(self.make_token(TokenType::EOF, self.source.len()))
    }

    fn make_token(&mut self, token_type: TokenType, start: usize) -> Token<'a> {
        let source = self.token_contents(start);
        Token::new(
            token_type,
            source,
            Position::new(start, start + source.len(), self.line),
        )
    }

    fn token_contents(&mut self, start: usize) -> &'a str {
        let end = self
            .chars
            .peek()
            .map(|&(i, _)| i)
            .unwrap_or(self.source.len());
        &self.source[start..end].trim_end()
    }

    fn skip_whitespace(&mut self) {
        self.advance_while(|&c| c == ' ' || c == '\t');
    }

    fn skip_lines(&mut self) {
        self.advance_while(|&c| c == ';' || c == '\n' || c == '\r');
    }

    fn advance_while<F>(&mut self, f: F) -> usize
        where
                for<'r> F: Fn(&'r char) -> bool,
    {
        let mut count = 0;
        while let Some(char) = self.peek() {
            if f(&char) {
                self.advance();
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        self.chars.next().map(|(current, c)| {
            if c == '\n' {
                self.line += 1;
            }
            (current, c)
        })
    }

    fn check(&mut self, c: char) -> Result<bool> {
        self.peek()
            .map(|p| p == c)
            .ok_or(SyntaxError::UnexpectedEOF)
    }

    fn peek_next(&mut self) -> Option<char> {
        self.chars.nth(1).map(|(_, c)| c)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_numbers() {
        let expect = vec![
            Token::new(TokenType::Number, "2", Position::new(0, 1, 1)),
            Token::new(TokenType::Number, "10", Position::new(2, 4, 1)),
            Token::new(TokenType::Number, "3.33", Position::new(5, 9, 1)),
            Token::new(TokenType::EOF, "", Position::new(9, 9, 1)),
        ];

        let source = r#"2 10 3.33"#;

        let actual = lex(source).unwrap();
        assert_eq!(expect, actual);
    }

    #[test]
    fn lex_strings() {
        let expect = vec![
            Token::new(TokenType::String, "\"Hello\"", Position::new(0, 7, 1)),
            Token::new(TokenType::String, "\",\"", Position::new(8, 11, 1)),
            Token::new(TokenType::String, "\"World!\"", Position::new(12, 20, 1)),
            Token::new(TokenType::EOF, "", Position::new(20, 20, 1)),
        ];

        let source = r#""Hello" "," "World!""#;

        let actual = lex(source).unwrap();
        assert_eq!(expect, actual);
    }

    #[test]
    fn lex_keywords() {
        let expect = vec![
            Token::new(
                TokenType::Keyword(Keyword::Let),
                "let",
                Position::new(0, 3, 1),
            ),
            Token::new(TokenType::Line, "", Position::new(3, 3, 2)),
            Token::new(TokenType::Identifier, "x", Position::new(16, 17, 2)),
            Token::new(TokenType::EOF, "", Position::new(17, 17, 2)),
        ];

        let source = r#"let
            x"#;

        let actual = lex(source).unwrap();
        assert_eq!(expect, actual);
    }

    #[test]
    fn lex_lines() {
        let expect = vec![
            Token::new(
                TokenType::Keyword(Keyword::Let),
                "let",
                Position::new(0, 3, 1),
            ),
            Token::new(TokenType::Identifier, "x", Position::new(4, 5, 1)),
            Token::new(TokenType::Equal, "=", Position::new(6, 7, 1)),
            Token::new(TokenType::Number, "3", Position::new(8, 9, 1)),
            Token::new(TokenType::Line, "", Position::new(9, 9, 2)),
            Token::new(
                TokenType::Keyword(Keyword::Let),
                "let",
                Position::new(22, 25, 2),
            ),
            Token::new(TokenType::Identifier, "y", Position::new(26, 27, 2)),
            Token::new(TokenType::Equal, "=", Position::new(28, 29, 2)),
            Token::new(TokenType::Number, "5", Position::new(30, 31, 2)),
            Token::new(TokenType::EOF, "", Position::new(31, 31, 2)),
        ];

        let source = r#"let x = 3
            let y = 5"#;

        let actual = lex(source).unwrap();
        assert_eq!(expect, actual);
    }
}

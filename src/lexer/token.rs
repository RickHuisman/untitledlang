#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    token_type: TokenType,
    source: &'a str,
    position: Position,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, source: &'a str, position: Position) -> Self {
        Token {
            token_type,
            source,
            position,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}

#[derive(Debug, PartialEq, Clone)] // TODO Clone
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Slash,
    Semicolon,

    // Literals
    String,
    Number,

    // Keywords
    Let,
    True,
    False,
    Fun,
    While,
    For,
    If,
    Else,
    Print,
    Return,

    Identifier,

    EOF,
}

pub trait ToKeyword {
    fn to_keyword(self) -> Option<TokenType>;
}

impl ToKeyword for &str {
    fn to_keyword(self) -> Option<TokenType> {
        Some(match self {
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            _ => return None,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Position {
    start: usize,
    end: usize,
    line: usize,
}

impl Position {
    pub fn new(start: usize, end: usize, line: usize) -> Self {
        Position { start, end, line }
    }

    pub fn start(&self) -> &usize {
        &self.start
    }

    pub fn end(&self) -> &usize {
        &self.end
    }

    pub fn line(&self) -> &usize {
        &self.line
    }
}

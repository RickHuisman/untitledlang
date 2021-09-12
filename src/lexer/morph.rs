use crate::lexer::token::{Position, Token, TokenType};

/// Cleans a sequence of tokens into a token sequence of meaningful tokens.
/// Tokens that are removed from the sequence:
/// - Comments
/// - Unessential lines
pub fn morph(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut morphed = vec![];

    // Removes leading lines.
    'foo: loop {
        if let Some(tk) = tokens.first() {
            match tk.token_type() {
                TokenType::Line => tokens.drain(0..1), // TODO: cleanup.
                _ => break 'foo,
            };
        };
    }

    while !tokens.is_empty() {
        let token = tokens.pop().unwrap();
        match token.token_type() {
            TokenType::Line => {
                if morphed.is_empty() {
                    morphed.push(token);
                } else {
                    let last_token_type = morphed.last().unwrap().token_type();
                    if last_token_type != &TokenType::Line {
                        morphed.push(token);
                    }
                }
            }
            _ => morphed.push(token),
        }
    }

    morphed.reverse();

    // TODO: Cleanup.
    // Add a final line to token sequence for 1 line expressions.
    let eof = morphed.pop().unwrap();
    morphed.push(Token::new(
        TokenType::Line,
        "",
        Position::new(
            eof.position().end().clone(),
            eof.position().end().clone(),
            eof.position().line().clone(),
        ),
    ));
    morphed.push(eof);

    morphed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::lexer::token::Position;

    #[test]
    fn morph_comments() {
        let expect = vec![
            Token::new(TokenType::Print, "print", Position::new(36, 41, 3)),
            Token::new(TokenType::LeftParen, "(", Position::new(41, 42, 3)),
            Token::new(TokenType::Number, "10", Position::new(42, 44, 3)),
            Token::new(TokenType::RightParen, ")", Position::new(44, 45, 3)),
            Token::new(TokenType::Line, "", Position::new(45, 45, 4)),
            Token::new(TokenType::Line, "", Position::new(54, 54, 4)),
            Token::new(TokenType::EOF, "", Position::new(54, 54, 4)),
        ];

        let source = r#"
        // This is a test!
        print(10)
        "#;

        let actual = lex(source).unwrap();
        assert_eq!(expect, actual);
    }
}

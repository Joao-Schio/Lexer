use std::collections::HashMap;

use crate::{
    lexer::{Lexer, TLexer},
    scanner::TScanner,
    token::TokenType,
};

struct DummyScanner {
    input: Vec<u8>,
    position: usize,
    line: usize,
    column: usize,
}

impl DummyScanner {
    fn new(input: &str) -> Self {
        Self {
            input: input.as_bytes().to_vec(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
}

impl TScanner for DummyScanner {
    fn get_line(&self) -> usize {
        self.line
    }

    fn get_next(&mut self) -> std::io::Result<Option<u8>> {
        let value = self.peek_next();

        if let Some(byte) = value {
            self.position += 1;

            if byte == b'\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }

        Ok(value)
    }

    fn peek_next(&self) -> Option<u8> {
        self.input.get(self.position).copied()
    }

    fn get_column(&self) -> usize {
        self.column
    }
}

mod contract {
    use super::*;

    fn assert_token<L: TLexer>(lexer: &mut L, expected_type: TokenType, expected_lexeme: &str) {
        let token = lexer.get_prox_token();

        assert_eq!(token.get_tok_type(), &expected_type);
        assert_eq!(token.get_lexema(), expected_lexeme);
    }

    pub fn recognizes_div<F, L>(make_lexer: F)
    where
        F: FnOnce(&str) -> L,
        L: TLexer,
    {
        let mut lexer = make_lexer("/");

        assert_token(&mut lexer, TokenType::Div, "/");
    }

    pub fn division_does_not_consume_following_token<F, L>(make_lexer: F)
    where
        F: FnOnce(&str) -> L,
        L: TLexer,
    {
        let mut lexer = make_lexer("/+");

        assert_token(&mut lexer, TokenType::Div, "/");
        assert_token(&mut lexer, TokenType::Plus, "+");
    }

    pub fn skips_empty_line_comment<F, L>(make_lexer: F)
    where
        F: FnOnce(&str) -> L,
        L: TLexer,
    {
        let mut lexer = make_lexer("//\n+");

        assert_token(&mut lexer, TokenType::Plus, "+");
    }

    pub fn skips_line_comment_with_content<F, L>(make_lexer: F)
    where
        F: FnOnce(&str) -> L,
        L: TLexer,
    {
        let mut lexer = make_lexer("//abc\n+");

        assert_token(&mut lexer, TokenType::Plus, "+");
    }

    pub fn skips_comment_between_tokens<F, L>(make_lexer: F)
    where
        F: FnOnce(&str) -> L,
        L: TLexer,
    {
        let mut lexer = make_lexer("+//abc\n-");

        assert_token(&mut lexer, TokenType::Plus, "+");
        assert_token(&mut lexer, TokenType::Minus, "-");
    }

    pub fn skips_consecutive_comments<F, L>(make_lexer: F)
    where
        F: FnOnce(&str) -> L,
        L: TLexer,
    {
        let mut lexer = make_lexer("//a\n//b\n+");

        assert_token(&mut lexer, TokenType::Plus, "+");
    }
}

fn make_lexer(input: &str) -> Lexer<DummyScanner> {
    Lexer::new(DummyScanner::new(input), HashMap::new())
}

#[test]
fn recognizes_div() {
    contract::recognizes_div(make_lexer);
}

#[test]
fn division_does_not_consume_following_token() {
    contract::division_does_not_consume_following_token(make_lexer);
}

#[test]
fn skips_empty_line_comment() {
    contract::skips_empty_line_comment(make_lexer);
}

#[test]
fn skips_line_comment_with_content() {
    contract::skips_line_comment_with_content(make_lexer);
}

#[test]
fn skips_comment_between_tokens() {
    contract::skips_comment_between_tokens(make_lexer);
}

#[test]
fn skips_consecutive_comments() {
    contract::skips_consecutive_comments(make_lexer);
}

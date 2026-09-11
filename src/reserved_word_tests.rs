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
    fn peek_next(&self) -> Option<u8> {
        self.input.get(self.position).copied()
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

    fn get_line(&self) -> usize {
        self.line
    }

    fn get_column(&self) -> usize {
        self.column
    }
}

fn assert_reserved_word(input: &'static str, expected_type: TokenType) {
    let reserved_words = HashMap::from([(input, expected_type)]);
    let mut lexer = Lexer::new(DummyScanner::new(input), reserved_words);
    let token = lexer.get_prox_token();

    assert_eq!(token.get_tok_type(), &expected_type);
    assert_eq!(token.get_lexema(), input);
}

#[test]
fn recognizes_main_reserved_word() {
    assert_reserved_word("main", TokenType::Main);
}

#[test]
fn recognizes_if_reserved_word() {
    assert_reserved_word("if", TokenType::If);
}

#[test]
fn recognizes_else_reserved_word() {
    assert_reserved_word("else", TokenType::Else);
}

#[test]
fn recognizes_for_reserved_word() {
    assert_reserved_word("for", TokenType::For);
}

#[test]
fn recognizes_return_reserved_word() {
    assert_reserved_word("return", TokenType::Return);
}

#[test]
fn recognizes_int_reserved_word() {
    assert_reserved_word("int", TokenType::Int);
}

#[test]
fn recognizes_char_reserved_word() {
    assert_reserved_word("char", TokenType::Char);
}

#[test]
fn recognizes_print_reserved_word() {
    assert_reserved_word("print", TokenType::Print);
}

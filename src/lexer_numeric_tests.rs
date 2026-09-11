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

fn make_lexer(input: &str) -> Lexer<DummyScanner> {
    Lexer::new(DummyScanner::new(input), HashMap::new())
}

fn assert_token<L: TLexer>(lexer: &mut L, expected_type: TokenType, expected_lexeme: &str) {
    let token = lexer.get_prox_token();

    assert_eq!(token.get_tok_type(), &expected_type);
    assert_eq!(token.get_lexema(), expected_lexeme);
}

#[test]
fn recognizes_zero() {
    let mut lexer = make_lexer("0");

    assert_token(&mut lexer, TokenType::IntegerConst(0), "0");
}

#[test]
fn recognizes_nine() {
    let mut lexer = make_lexer("9");

    assert_token(&mut lexer, TokenType::IntegerConst(9), "9");
}

#[test]
fn recognizes_multi_digit_integer() {
    let mut lexer = make_lexer("12341235");

    assert_token(
        &mut lexer,
        TokenType::IntegerConst(12_341_235),
        "12341235",
    );
}

#[test]
fn preserves_leading_zeroes_in_integer_lexeme() {
    let mut lexer = make_lexer("00123");

    assert_token(&mut lexer, TokenType::IntegerConst(123), "00123");
}

#[test]
fn integer_does_not_consume_following_operator() {
    let mut lexer = make_lexer("123+");

    assert_token(&mut lexer, TokenType::IntegerConst(123), "123");
    assert_token(&mut lexer, TokenType::Plus, "+");
}

#[test]
fn integer_does_not_consume_following_identifier() {
    let mut lexer = make_lexer("123abc");

    assert_token(&mut lexer, TokenType::IntegerConst(123), "123");
    assert_token(&mut lexer, TokenType::Id, "abc");
}

#[test]
fn scientific_notation_is_not_an_integer_literal() {
    let mut lexer = make_lexer("12E2");

    assert_token(&mut lexer, TokenType::IntegerConst(12), "12");
    assert_token(&mut lexer, TokenType::Id, "E2");
}

#[test]
fn negative_value_is_minus_followed_by_integer() {
    let mut lexer = make_lexer("-123");

    assert_token(&mut lexer, TokenType::Minus, "-");
    assert_token(&mut lexer, TokenType::IntegerConst(123), "123");
}

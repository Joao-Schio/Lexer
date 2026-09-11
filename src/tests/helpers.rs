use std::collections::HashMap;

use crate::{
    lexer::{Lexer, TLexer},
    scanner::{ScannerError, TScanner},
    token::TokenType,
};

pub(super) struct DummyScanner {
    input: Vec<u8>,
    position: usize,
    line: usize,
    column: usize,
}

impl DummyScanner {
    pub(super) fn new(input: &str) -> Self {
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

    fn get_next(&mut self) -> Result<Option<u8>, ScannerError> {
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

pub(super) fn make_lexer(input: &str) -> Lexer<DummyScanner> {
    Lexer::new(DummyScanner::new(input), HashMap::new())
}

pub(super) fn make_lexer_with_reserved_words(
    input: &str,
    reserved_words: HashMap<&'static str, TokenType>,
) -> Lexer<DummyScanner> {
    Lexer::new(DummyScanner::new(input), reserved_words)
}

pub(super) fn assert_token<L: TLexer>(
    lexer: &mut L,
    expected_type: TokenType,
    expected_lexeme: &str,
) {
    let token = lexer.get_prox_token();

    assert_eq!(token.get_tok_type(), &expected_type);
    assert_eq!(token.get_lexema(), expected_lexeme);
}

pub(super) fn assert_token_type<L: TLexer>(lexer: &mut L, expected_type: TokenType) {
    let token = lexer.get_prox_token();

    assert_eq!(token.get_tok_type(), &expected_type);
}

use core::panic;

use crate::{
    scanner::TScanner,
    token::{Token, TokenType},
};

pub trait TLexer {
    fn get_prox_token(&mut self) -> Token;
}

pub struct Lexer<S: TScanner> {
    scanner: S,
}

impl<S: TScanner> Lexer<S> {
    pub fn new(scanner: S) -> Self {
        Self { scanner }
    }

    fn lex_token(&mut self, initial: u8) -> Token {
        match initial {
            b'+' => self.single_char_token(TokenType::Plus, "+"),
            b'-' => self.single_char_token(TokenType::Minus, "-"),
            _ => todo!(),
        }
    }

    fn single_char_token(&self, token_type: TokenType, lexeme: &str) -> Token {
        Token::new(token_type, self.scanner.get_line(), lexeme.to_owned())
    }
}

impl<S: TScanner> TLexer for Lexer<S> {
    fn get_prox_token(&mut self) -> Token {
        let initial = self
            .scanner
            .get_next()
            .expect("Io failed")
            .expect("EOF handling later");
        return self.lex_token(initial);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Lexer, TLexer},
        scanner::TScanner,
        token::TokenType,
    };

    struct DummyScanner {
        input: Vec<u8>,
        position: usize,
        line: usize,
    }

    impl DummyScanner {
        fn new(input: &str) -> Self {
            Self {
                input: input.as_bytes().to_vec(),
                position: 0,
                line: 1,
            }
        }
    }

    impl TScanner for DummyScanner {
        fn get_line(&self) -> usize {
            self.line
        }

        fn get_next(&mut self) -> std::io::Result<Option<u8>> {
            let value = self.peek_next();

            if value.is_some() {
                self.position += 1;
            }

            Ok(value)
        }

        fn peek_next(&self) -> Option<u8> {
            self.input.get(self.position).copied()
        }
    }

    mod contract {
        use super::*;

        pub fn recognizes_plus<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("+");

            let token = lexer.get_prox_token();

            assert_eq!(token.get_tok_type(), &TokenType::Plus);
            assert_eq!(token.get_lexema(), "+");
        }

        pub fn recognizes_minus<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("-");

            let token = lexer.get_prox_token();

            assert_eq!(token.get_tok_type(), &TokenType::Minus);
            assert_eq!(token.get_lexema(), "-");
        }
    }

    mod lexer_contract_tests {
        use super::*;

        fn make_lexer(input: &str) -> Lexer<DummyScanner> {
            Lexer::new(DummyScanner::new(input))
        }

        #[test]
        fn recognizes_plus() {
            contract::recognizes_plus(make_lexer);
        }

        #[test]
        fn recognizes_minus() {
            contract::recognizes_minus(make_lexer);
        }
    }
}

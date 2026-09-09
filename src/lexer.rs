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
            b'=' => self.match_equal(),
            b'>' => self.match_greater(),
            _ => todo!(),
        }
    }

    fn discard_next(&mut self) {
        let _ = self.scanner.get_next();
    }

    fn single_char_token(&self, token_type: TokenType, lexeme: &str) -> Token {
        Token::new(token_type, self.scanner.get_line(), lexeme.to_owned())
    }

    fn match_equal(&mut self) -> Token {
        if self.scanner.peek_next() != Some(b'=') {
            return Token::new(TokenType::Assign, self.scanner.get_line(), "=".to_string());
        }
        self.discard_next();
        Token::new(TokenType::Eq, self.scanner.get_line(), "==".to_string())
    }

    fn match_greater(&mut self) -> Token {
        if self.scanner.peek_next() != Some(b'=') {
            return Token::new(TokenType::Gt, self.scanner.get_line(), ">".to_string());
        }
        self.discard_next();
        Token::new(TokenType::Geq, self.scanner.get_line(), ">=".to_string())
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

        pub fn recognizes_simple_assignment<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("=");

            let token = lexer.get_prox_token();

            assert_eq!(token.get_tok_type(), &TokenType::Assign);
            assert_eq!(token.get_lexema(), "=");
        }

        pub fn recognizes_complex_assignment<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("=123"); // prox token deve ser assign

            let token = lexer.get_prox_token();

            assert_eq!(token.get_tok_type(), &TokenType::Assign);
            assert_eq!(token.get_lexema(), "=");
        }

        pub fn recognizes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("==");

            let token = lexer.get_prox_token();

            assert_eq!(token.get_tok_type(), &TokenType::Eq);
            assert_eq!(token.get_lexema(), "==");
        }

        pub fn recognizes_greater_than<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(">");

            let token = lexer.get_prox_token();

            assert_eq!(token.get_tok_type(), &TokenType::Gt);
            assert_eq!(token.get_lexema(), ">");
        }

        pub fn recognizes_greater_or_eq<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(">=");

            let token = lexer.get_prox_token();

            assert_eq!(token.get_tok_type(), &TokenType::Geq);
            assert_eq!(token.get_lexema(), ">=");
        }

        pub fn equality_consumes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("==+");

            let equality = lexer.get_prox_token();
            assert_eq!(equality.get_tok_type(), &TokenType::Eq);

            let plus = lexer.get_prox_token();
            assert_eq!(plus.get_tok_type(), &TokenType::Plus);
        }

        pub fn geq_consumes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(">=+");

            let equality = lexer.get_prox_token();
            assert_eq!(equality.get_tok_type(), &TokenType::Geq);

            let plus = lexer.get_prox_token();
            assert_eq!(plus.get_tok_type(), &TokenType::Plus);
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

        #[test]
        fn recognizes_simple_assignment() {
            contract::recognizes_simple_assignment(make_lexer);
        }

        #[test]
        fn recognizes_equals() {
            contract::recognizes_equals(make_lexer);
        }

        #[test]
        fn recognizes_complex_assignment() {
            contract::recognizes_complex_assignment(make_lexer);
        }

        #[test]
        fn recognizes_greater_than() {
            contract::recognizes_greater_than(make_lexer);
        }

        #[test]
        fn recognizes_greater_or_equal() {
            contract::recognizes_greater_or_eq(make_lexer);
        }

        #[test]
        fn equality_consumes_equal() {
            contract::equality_consumes_equals(make_lexer);
        }

        #[test]
        fn geq_consumes_equals() {
            contract::geq_consumes_equals(make_lexer);
        }
    }
}

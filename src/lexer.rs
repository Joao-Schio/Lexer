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
            b'<' => self.match_lesser(),
            _ => todo!(),
        }
    }

    fn discard_next(&mut self) {
        let _ = self.scanner.get_next().expect("IO Error detected");
    }

    fn single_char_token(&self, token_type: TokenType, lexeme: &str) -> Token {
        Token::new(token_type, self.scanner.get_line(), lexeme.to_owned())
    }

    fn match_optional_equal(
        &mut self,
        single_type: TokenType,
        equal_type: TokenType,
        single_lexeme: &str,
        equal_lexeme: &str,
    ) -> Token {
        if self.scanner.peek_next() != Some(b'=') {
            return Token::new(
                single_type,
                self.scanner.get_line(),
                single_lexeme.to_owned(),
            );
        }

        self.discard_next();
        Token::new(equal_type, self.scanner.get_line(), equal_lexeme.to_owned())
    }

    fn match_equal(&mut self) -> Token {
        self.match_optional_equal(TokenType::Assign, TokenType::Eq, "=", "==")
    }

    fn match_greater(&mut self) -> Token {
        self.match_optional_equal(TokenType::Gt, TokenType::Geq, ">", ">=")
    }

    fn match_lesser(&mut self) -> Token {
        self.match_optional_equal(TokenType::Lt, TokenType::Leq, "<", "<=")
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

        fn assert_token<L: TLexer>(lexer: &mut L, expected_type: TokenType, expected_lexeme: &str) {
            let token = lexer.get_prox_token();

            assert_eq!(token.get_tok_type(), &expected_type);
            assert_eq!(token.get_lexema(), expected_lexeme);
        }

        pub fn recognizes_plus<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("+");

            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn recognizes_minus<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("-");

            assert_token(&mut lexer, TokenType::Minus, "-");
        }

        pub fn recognizes_simple_assignment<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("=");

            assert_token(&mut lexer, TokenType::Assign, "=");
        }

        pub fn recognizes_complex_assignment<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("=123"); // prox token deve ser assign

            assert_token(&mut lexer, TokenType::Assign, "=");
        }

        pub fn recognizes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("==");

            assert_token(&mut lexer, TokenType::Eq, "==");
        }

        pub fn recognizes_greater_than<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(">");

            assert_token(&mut lexer, TokenType::Gt, ">");
        }

        pub fn recognizes_greater_or_eq<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(">=");

            assert_token(&mut lexer, TokenType::Geq, ">=");
        }

        pub fn equality_consumes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("==+");

            assert_token(&mut lexer, TokenType::Eq, "==");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn geq_consumes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(">=+");

            assert_token(&mut lexer, TokenType::Geq, ">=");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn recognizes_less_than<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("<");

            assert_token(&mut lexer, TokenType::Lt, "<");
        }

        pub fn recognizes_less_or_equal<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("<=");

            assert_token(&mut lexer, TokenType::Leq, "<=");
        }

        pub fn leq_consumes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("<=+");

            assert_token(&mut lexer, TokenType::Leq, "<=");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn less_than_does_not_consume_next<F, L>(make_lexer: F)
        where
            F: FnOnce(&str) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("<+");

            assert_token(&mut lexer, TokenType::Lt, "<");
            assert_token(&mut lexer, TokenType::Plus, "+");
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

        #[test]
        fn recognizes_less_than() {
            contract::recognizes_less_than(make_lexer);
        }

        #[test]
        fn recognizes_less_or_equal() {
            contract::recognizes_less_or_equal(make_lexer);
        }

        #[test]
        fn leq_consumes_equals() {
            contract::leq_consumes_equals(make_lexer);
        }

        #[test]
        fn less_than_does_not_consume_next() {
            contract::less_than_does_not_consume_next(make_lexer);
        }
    }
}

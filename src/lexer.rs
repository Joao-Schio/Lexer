use std::collections::HashMap;

use crate::{
    scanner::TScanner,
    token::{Token, TokenType},
};

pub trait TLexer {
    fn get_prox_token(&mut self) -> Token;
}

pub struct Lexer<S: TScanner> {
    scanner: S,
    reserved_words: HashMap<&'static str, TokenType>,
}

impl<S: TScanner> Lexer<S> {
    pub fn new(scanner: S, reserved_words: HashMap<&'static str, TokenType>) -> Self {
        Self {
            scanner,
            reserved_words,
        }
    }

    fn discard_comment(&mut self) {
        while let Some(c) = self.scanner.peek_next() {
            self.discard_next();
            if c == b'\n' {
                break;
            }
        }
    }

    fn get_next_meaningful_char(&mut self) -> Option<u8> {
        loop {
            let initial = self.scanner.get_next().expect("Io error");
            if initial.is_none() {
                return None;
            }
            let initial = initial.unwrap();
            if initial.is_ascii_whitespace() {
                continue;
            }
            if initial == b'/' && self.scanner.peek_next() == Some(b'/') {
                self.discard_next();
                self.discard_comment();
                continue;
            }
            return Some(initial);
        }
    }

    fn lex_token(&mut self, initial: u8) -> Token {
        match initial {
            b'+' => self.single_char_token(TokenType::Plus, "+"),
            b'-' => self.single_char_token(TokenType::Minus, "-"),
            b'*' => self.single_char_token(TokenType::Mul, "*"),
            b'%' => self.single_char_token(TokenType::Mod, "%"),
            b'/' => self.single_char_token(TokenType::Div, "/"),
            b'=' => self.match_equal(),
            b'>' => self.match_greater(),
            b'<' => self.match_lesser(),
            b'&' => self.match_and(),
            b'|' => self.match_or(),
            b'!' => self.match_not(),
            b'"' => self.match_quotes(),
            b'\'' => self.match_single_quote(),
            b',' => self.single_char_token(TokenType::Comma, ","),
            b';' => self.single_char_token(TokenType::SemiColon, ";"),
            b'(' => self.single_char_token(TokenType::Lparen, "("),
            b')' => self.single_char_token(TokenType::Rparen, ")"),
            b'{' => self.single_char_token(TokenType::LBrace, "{"),
            b'}' => self.single_char_token(TokenType::RBrace, "}"),
            b'[' => self.single_char_token(TokenType::LBracket, "["),
            b']' => self.single_char_token(TokenType::RBracket, "]"),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.match_letters_tokens(initial),
            b'0'..b'9' => self.match_numeric(initial),
            _ => todo!(),
        }
    }

    fn is_allowed_identifier_character(character: u8) -> bool {
        match character {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9' => true,
            _ => false,
        }
    }
    fn is_allowed_numeric_character(character: u8) -> bool {
        match character {
            b'0'..=b'9' => true,
            _ => false,
        }
    }

    fn match_letters_tokens(&mut self, initial: u8) -> Token {
        let mut id = String::from(initial as char);
        while let Some(next) = self.scanner.peek_next() {
            if !Self::is_allowed_identifier_character(next) {
                break;
            }

            self.discard_next();
            id.push(next as char);
        }
        let tipo = self
            .reserved_words
            .get(id.as_str())
            .copied()
            .unwrap_or(TokenType::Id);

        Token::new(tipo, self.scanner.get_line(), id)
    }

    fn match_single_quote(&mut self) -> Token {
        let line = self.scanner.get_line();

        let Some(byte) = self.scanner.get_next().expect("I/O error") else {
            return Token::new(TokenType::Undef, line, "'".into());
        };

        if byte == b'\'' || byte == b'\n' {
            return Token::new(TokenType::Undef, line, format!("'{}", byte as char));
        }

        let character = byte as char;

        match self.scanner.get_next().expect("I/O error") {
            Some(b'\'') => Token::new(TokenType::CharConst, line, character.to_string()),

            Some(next) => Token::new(
                TokenType::Undef,
                line,
                format!("'{character}{}", next as char),
            ),

            None => Token::new(TokenType::Undef, line, format!("'{character}")),
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

    fn match_not(&mut self) -> Token {
        self.match_optional_equal(TokenType::Not, TokenType::Neq, "!", "!=")
    }

    fn match_and(&mut self) -> Token {
        self.match_required_pair(b'&', TokenType::And, "&", "&&")
    }

    fn match_or(&mut self) -> Token {
        self.match_required_pair(b'|', TokenType::Or, "|", "||")
    }

    fn match_required_pair(
        &mut self,
        expected: u8,
        token_type: TokenType,
        single_lexeme: &str,
        pair_lexeme: &str,
    ) -> Token {
        if self.scanner.peek_next() != Some(expected) {
            return Token::new(
                TokenType::Undef,
                self.scanner.get_line(),
                single_lexeme.to_owned(),
            );
        }

        self.discard_next();

        Token::new(token_type, self.scanner.get_line(), pair_lexeme.to_owned())
    }

    fn match_quotes(&mut self) -> Token {
        let line = self.scanner.get_line();
        let mut buffer = String::new();

        loop {
            match self.scanner.get_next().expect("IO Error detected") {
                Some(b'"') => {
                    return Token::new(TokenType::StringConst, line, buffer);
                }

                Some(b'\n') | None => {
                    return Token::new(TokenType::Undef, line, buffer);
                }

                Some(c) => buffer.push(c as char),
            }
        }
    }

    fn match_numeric(&mut self, initial: u8) -> Token {
        let mut buffer = String::from(initial as char);
        let linha = self.scanner.get_line();
        while let Some(next) = self.scanner.peek_next() {
            buffer.push(next as char);
            if Self::is_allowed_numeric_character(next) {
                self.discard_next();
            } else {
                return Token::new(TokenType::Undef, linha, buffer);
            }
        }
        Token::new(
            TokenType::IntegerConst(buffer.parse().unwrap()),
            linha,
            buffer,
        )
    }
}

impl<S: TScanner> TLexer for Lexer<S> {
    fn get_prox_token(&mut self) -> Token {
        let initial = self.get_next_meaningful_char().expect("eof handling later");
        self.lex_token(initial)
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
        use std::collections::HashMap;

        use super::*;

        fn assert_token<L: TLexer>(lexer: &mut L, expected_type: TokenType, expected_lexeme: &str) {
            let token = lexer.get_prox_token();

            assert_eq!(token.get_tok_type(), &expected_type);
            assert_eq!(token.get_lexema(), expected_lexeme);
        }

        pub fn recognizes_simple_assignment<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("=", None);

            assert_token(&mut lexer, TokenType::Assign, "=");
        }

        pub fn recognizes_complex_assignment<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("=123", None); // prox token deve ser assign

            assert_token(&mut lexer, TokenType::Assign, "=");
        }

        pub fn recognizes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("==", None);

            assert_token(&mut lexer, TokenType::Eq, "==");
        }

        pub fn recognizes_greater_than<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(">", None);

            assert_token(&mut lexer, TokenType::Gt, ">");
        }

        pub fn recognizes_greater_or_eq<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(">=", None);

            assert_token(&mut lexer, TokenType::Geq, ">=");
        }

        pub fn equality_consumes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("==+", None);

            assert_token(&mut lexer, TokenType::Eq, "==");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn geq_consumes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(">=+", None);

            assert_token(&mut lexer, TokenType::Geq, ">=");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn recognizes_less_than<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("<", None);

            assert_token(&mut lexer, TokenType::Lt, "<");
        }

        pub fn recognizes_less_or_equal<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("<=", None);

            assert_token(&mut lexer, TokenType::Leq, "<=");
        }

        pub fn leq_consumes_equals<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("<=+", None);

            assert_token(&mut lexer, TokenType::Leq, "<=");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn less_than_does_not_consume_next<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("<+", None);

            assert_token(&mut lexer, TokenType::Lt, "<");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn recognizes_logical_and<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("&&", None);

            assert_token(&mut lexer, TokenType::And, "&&");
        }

        pub fn logical_and_consumes_both_characters<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("&&+", None);

            assert_token(&mut lexer, TokenType::And, "&&");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn single_ampersand_is_error<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("&", None);

            assert_token(&mut lexer, TokenType::Undef, "&");
        }

        pub fn invalid_ampersand_does_not_consume_next<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("&+", None);

            assert_token(&mut lexer, TokenType::Undef, "&");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn recognizes_logical_or<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("||", None);

            assert_token(&mut lexer, TokenType::Or, "||");
        }

        pub fn single_pipe_is_error<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("|", None);

            assert_token(&mut lexer, TokenType::Undef, "|");
        }

        pub fn invalid_pipe_does_not_consume_next<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("|+", None);

            assert_token(&mut lexer, TokenType::Undef, "|");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn recognizes_not<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("!", None);

            assert_token(&mut lexer, TokenType::Not, "!");
        }

        pub fn recognizes_not_equal<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("!=", None);

            assert_token(&mut lexer, TokenType::Neq, "!=");
        }

        pub fn neq_consumes_equal<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("!=+", None);

            assert_token(&mut lexer, TokenType::Neq, "!=");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }
        fn assert_token_type<L: TLexer>(lexer: &mut L, expected_type: TokenType) {
            let token = lexer.get_prox_token();

            assert_eq!(token.get_tok_type(), &expected_type);
        }

        pub fn recognizes_char_const<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("'a'", None);

            assert_token(&mut lexer, TokenType::CharConst, "a");
        }

        pub fn recognizes_symbol_char_const<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("'+'", None);

            assert_token(&mut lexer, TokenType::CharConst, "+");
        }

        pub fn char_const_consumes_closing_quote<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("'a'+", None);

            assert_token(&mut lexer, TokenType::CharConst, "a");

            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn unterminated_char_const_is_error<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("'a", None);

            assert_token_type(&mut lexer, TokenType::Undef);
        }

        pub fn empty_char_const_is_error<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("''", None);

            assert_token_type(&mut lexer, TokenType::Undef);
        }

        pub fn multiple_character_char_const_is_error<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("'ab'", None);

            assert_token_type(&mut lexer, TokenType::Undef);
        }

        pub fn newline_in_char_const_is_error<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("'\n'", None);

            assert_token_type(&mut lexer, TokenType::Undef);
        }

        pub fn recognizes_string_const<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("\"hello\"", None);

            assert_token(&mut lexer, TokenType::StringConst, "hello");
        }

        pub fn recognizes_empty_string_const<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("\"\"", None);

            assert_token(&mut lexer, TokenType::StringConst, "");
        }

        pub fn recognizes_string_const_with_symbols<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("\"123 !@#$%\"", None);

            assert_token(&mut lexer, TokenType::StringConst, "123 !@#$%");
        }

        pub fn string_const_consumes_closing_quote<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("\"hello\"+", None);

            assert_token(&mut lexer, TokenType::StringConst, "hello");

            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn unterminated_string_const_is_error<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("\"hello", None);

            assert_token_type(&mut lexer, TokenType::Undef);
        }

        pub fn newline_in_string_const_is_error<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("\"hello\nworld\"", None);

            assert_token_type(&mut lexer, TokenType::Undef);
        }

        pub fn recognizes_identifier<F, L>(make_lexer: F, input: &str)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(input, None);

            assert_token(&mut lexer, TokenType::Id, input);
        }

        pub fn recognizes_reserved_word<F, L>(
            make_lexer: F,
            input: &'static str,
            expected_type: TokenType,
        ) where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let reserved_words = HashMap::from([(input, expected_type)]);
            let mut lexer = make_lexer(input, Some(reserved_words));

            assert_token(&mut lexer, expected_type, input);
        }

        pub fn reserved_word_match_is_exact<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let reserved_words = HashMap::from([("if", TokenType::If)]);
            let mut lexer = make_lexer("ifx", Some(reserved_words));

            assert_token(&mut lexer, TokenType::Id, "ifx");
        }

        pub fn identifier_does_not_consume_following_token<F, L>(make_lexer: F)
        where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer("value+", None);

            assert_token(&mut lexer, TokenType::Id, "value");
            assert_token(&mut lexer, TokenType::Plus, "+");
        }

        pub fn recognizes_single_char_token<F, L>(
            make_lexer: F,
            input: &str,
            expected_type: TokenType,
        ) where
            F: FnOnce(&str, Option<HashMap<&'static str, TokenType>>) -> L,
            L: TLexer,
        {
            let mut lexer = make_lexer(input, None);

            assert_token(&mut lexer, expected_type, input);
        }
    }

    mod lexer_contract_tests {
        use std::collections::HashMap;

        use super::*;

        fn make_lexer(
            input: &str,
            reserved_words: Option<HashMap<&'static str, TokenType>>,
        ) -> Lexer<DummyScanner> {
            let reserved_words = reserved_words.unwrap_or(HashMap::new());
            Lexer::new(DummyScanner::new(input), reserved_words)
        }

        #[test]
        fn recognizes_plus() {
            contract::recognizes_single_char_token(make_lexer, "+", TokenType::Plus);
        }

        #[test]
        fn recognizes_minus() {
            contract::recognizes_single_char_token(make_lexer, "-", TokenType::Minus);
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

        #[test]
        fn recognizes_logical_and() {
            contract::recognizes_logical_and(make_lexer);
        }

        #[test]
        fn logical_and_consumes_both_characters() {
            contract::logical_and_consumes_both_characters(make_lexer);
        }

        #[test]
        fn single_ampersand_is_error() {
            contract::single_ampersand_is_error(make_lexer);
        }

        #[test]
        fn invalid_ampersand_does_not_consume_next() {
            contract::invalid_ampersand_does_not_consume_next(make_lexer);
        }

        #[test]
        fn recognizes_logical_or() {
            contract::recognizes_logical_or(make_lexer);
        }

        #[test]
        fn recognizes_not() {
            contract::recognizes_not(make_lexer);
        }

        #[test]
        fn recognizes_not_equal() {
            contract::recognizes_not_equal(make_lexer);
        }
        #[test]
        fn recognizes_char_const() {
            contract::recognizes_char_const(make_lexer);
        }

        #[test]
        fn recognizes_symbol_char_const() {
            contract::recognizes_symbol_char_const(make_lexer);
        }

        #[test]
        fn char_const_consumes_closing_quote() {
            contract::char_const_consumes_closing_quote(make_lexer);
        }

        #[test]
        fn unterminated_char_const_is_error() {
            contract::unterminated_char_const_is_error(make_lexer);
        }

        #[test]
        fn empty_char_const_is_error() {
            contract::empty_char_const_is_error(make_lexer);
        }

        #[test]
        fn multiple_character_char_const_is_error() {
            contract::multiple_character_char_const_is_error(make_lexer);
        }

        #[test]
        fn newline_in_char_const_is_error() {
            contract::newline_in_char_const_is_error(make_lexer);
        }

        #[test]
        fn recognizes_string_const() {
            contract::recognizes_string_const(make_lexer);
        }

        #[test]
        fn recognizes_empty_string_const() {
            contract::recognizes_empty_string_const(make_lexer);
        }

        #[test]
        fn recognizes_string_const_with_symbols() {
            contract::recognizes_string_const_with_symbols(make_lexer);
        }

        #[test]
        fn string_const_consumes_closing_quote() {
            contract::string_const_consumes_closing_quote(make_lexer);
        }

        #[test]
        fn unterminated_string_const_is_error() {
            contract::unterminated_string_const_is_error(make_lexer);
        }

        #[test]
        fn newline_in_string_const_is_error() {
            contract::newline_in_string_const_is_error(make_lexer);
        }

        #[test]
        fn recognizes_comma() {
            contract::recognizes_single_char_token(make_lexer, ",", TokenType::Comma);
        }

        #[test]
        fn recognizes_semicolon() {
            contract::recognizes_single_char_token(make_lexer, ";", TokenType::SemiColon);
        }

        #[test]
        fn recognizes_left_parenthesis() {
            contract::recognizes_single_char_token(make_lexer, "(", TokenType::Lparen);
        }

        #[test]
        fn recognizes_right_parenthesis() {
            contract::recognizes_single_char_token(make_lexer, ")", TokenType::Rparen);
        }

        #[test]
        fn recognizes_left_brace() {
            contract::recognizes_single_char_token(make_lexer, "{", TokenType::LBrace);
        }

        #[test]
        fn recognizes_right_brace() {
            contract::recognizes_single_char_token(make_lexer, "}", TokenType::RBrace);
        }

        #[test]
        fn recognizes_left_bracket() {
            contract::recognizes_single_char_token(make_lexer, "[", TokenType::LBracket);
        }

        #[test]
        fn recognizes_right_bracket() {
            contract::recognizes_single_char_token(make_lexer, "]", TokenType::RBracket);
        }

        #[test]
        fn recognizes_mul() {
            contract::recognizes_single_char_token(make_lexer, "*", TokenType::Mul);
        }

        #[test]
        fn recognizes_mod() {
            contract::recognizes_single_char_token(make_lexer, "%", TokenType::Mod);
        }

        #[test]
        fn recognizes_identifier() {
            contract::recognizes_identifier(make_lexer, "value");
        }

        #[test]
        fn recognizes_identifier_with_uppercase_digits_and_underscore() {
            contract::recognizes_identifier(make_lexer, "Value_123");
        }

        #[test]
        fn recognizes_identifier_starting_with_underscore() {
            contract::recognizes_identifier(make_lexer, "_value");
        }

        #[test]
        fn recognizes_injected_reserved_word() {
            contract::recognizes_reserved_word(make_lexer, "if", TokenType::If);
        }

        #[test]
        fn reserved_word_match_is_exact() {
            contract::reserved_word_match_is_exact(make_lexer);
        }

        #[test]
        fn identifier_does_not_consume_following_token() {
            contract::identifier_does_not_consume_following_token(make_lexer);
        }
    }
}

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
            b'0'..=b'9' => self.match_numeric(initial),
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
            if !Self::is_allowed_numeric_character(next) {
                break;
            }

            self.discard_next();
            buffer.push(next as char);
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

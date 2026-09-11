#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    IntegerConst(i64),
    CharConst,
    StringConst,
    Undef,
    Id,
    Eof,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    And,
    Or,
    Not,
    Assign,
    SemiColon,
    Comma,
    Lparen,
    Rparen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Main,
    If,
    Else,
    For,
    Return,
    Int,
    Char,
    Print,
}

pub struct Token {
    tipo: TokenType,
    linha: usize,
    lexema: String,
}

impl Token {
    pub fn new(tipo: TokenType, linha: usize, lexema: String) -> Self {
        Self {
            tipo,
            linha,
            lexema,
        }
    }

    pub fn get_tok_type(&self) -> &TokenType {
        &self.tipo
    }

    pub fn get_lexema(&self) -> &str {
        &self.lexema
    }
}

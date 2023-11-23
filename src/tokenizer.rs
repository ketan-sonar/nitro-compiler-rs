#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    IntLiteral,
    Ident,
    Exit,
    Let,
    LParen,
    RParen,
    Eq,
    SemiColon,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub value: Option<String>,
}

impl Token {
    pub fn new(type_: TokenType, value: Option<String>) -> Self {
        Self { type_, value }
    }

    pub fn new_int_literal_token(value: String) -> Self {
        Self::new(TokenType::IntLiteral, Some(value))
    }

    pub fn new_identifier_token(value: String) -> Self {
        Self::new(TokenType::Ident, Some(value))
    }

    pub fn new_exit_token() -> Self {
        Self::new(TokenType::Exit, None)
    }

    pub fn new_let_token() -> Self {
        Self::new(TokenType::Let, None)
    }

    pub fn new_l_paren_token() -> Self {
        Self::new(TokenType::LParen, None)
    }

    pub fn new_r_paren_token() -> Self {
        Self::new(TokenType::RParen, None)
    }

    pub fn new_eq_token() -> Self {
        Self::new(TokenType::Eq, None)
    }

    pub fn new_semi_colon_token() -> Self {
        Self::new(TokenType::SemiColon, None)
    }
}

pub struct Tokenizer {
    src: Vec<u8>,
    index: usize,
}

impl Tokenizer {
    pub fn new(src: Vec<u8>) -> Self {
        Self { src, index: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut buf = String::new();

        while self.peek().is_some() {
            let mut c = self.peek().unwrap();
            if c.is_ascii_alphabetic() {
                while c.is_ascii_alphabetic() {
                    buf.push(self.consume() as char);
                    if self.peek().is_none() { break; }
                    c = self.peek().unwrap();
                }

                if buf == "exit" {
                    tokens.push(Token::new_exit_token());
                    buf.clear();
                } else if buf == "let" {
                    tokens.push(Token::new_let_token());
                    buf.clear();
                } else {
                    tokens.push(Token::new_identifier_token(buf.clone()));
                    buf.clear();
                }
            } else if c.is_ascii_digit() {
                while c.is_ascii_digit() {
                    buf.push(self.consume() as char);
                    if self.peek().is_none() { break; }
                    c = self.peek().unwrap();
                }

                tokens.push(Token::new_int_literal_token(buf.clone()));
                buf.clear();
            } else if c == b'(' {
                tokens.push(Token::new_l_paren_token());
                self.consume();
            } else if c == b')' {
                tokens.push(Token::new_r_paren_token());
                self.consume();
            } else if c == b'=' {
                tokens.push(Token::new_eq_token());
                self.consume();
            } else if c == b';' {
                tokens.push(Token::new_semi_colon_token());
                self.consume();
            } else {
                self.consume();
            }
        }

        tokens
    }

    fn peek(&self) -> Option<u8> {
        if self.index >= self.src.len() {
            return None;
        }
        Some(self.src[self.index])
    }

    fn consume(&mut self) -> u8 {
        self.index += 1;
        self.src[self.index-1]
    }
}

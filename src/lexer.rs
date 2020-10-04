use crate::span::Span;
use TokenKind::*;

pub struct Lexer<'a> {
    source: &'a [u8],
    start: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            start: 0,
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        while !self.eof() {
            self.start = self.pos;
            let t = match self.next_char() {
                b'+' => self.token(Plus),
                b'-' => {
                    if self.eat(b'>') {
                        self.token(LArrow)
                    } else {
                        self.token(Minus)
                    }
                }
                b'*' => self.token(Star),
                b'/' => self.token(Slash),
                b'(' => self.token(OpenParen),
                b')' => self.token(CloseParen),
                b'{' => self.token(OpenBrace),
                b'}' => self.token(CloseBrace),
                b':' => self.token(Colon),
                b';' => self.token(Semicolon),
                b'=' => {
                    if self.eat(b'=') {
                        self.token(EqEq)
                    } else {
                        self.token(Eq)
                    }
                }
                b'!' => {
                    if self.eat(b'=') {
                        self.token(Ne)
                    } else {
                        self.token(Not)
                    }
                }
                b'>' => {
                    if self.eat(b'=') {
                        self.token(Ge)
                    } else {
                        self.token(Gt)
                    }
                }
                b'<' => {
                    if self.eat(b'=') {
                        self.token(Le)
                    } else {
                        self.token(Lt)
                    }
                }
                b'&' => {
                    if self.eat(b'&') {
                        self.token(AndAnd)
                    } else {
                        self.token(And)
                    }
                }
                b'|' => {
                    if self.eat(b'|') {
                        self.token(OrOr)
                    } else {
                        self.token(Or)
                    }
                }
                c if c.is_ascii_alphabetic() => self.ident_or_kw(),
                c if c.is_ascii_digit() => self.number(),
                c if c.is_ascii_whitespace() => continue,
                c => panic!("Unexpected character {}", c as char),
            };
            return t;
        }

        self.token(Eof)
    }

    fn ident_or_kw(&mut self) -> Token {
        self.eat_while(|c| c.is_ascii_alphanumeric() || c == b'_');
        let span = self.span();
        let lexeme = &self.source[span.lo..span.hi];
        let kind = match lexeme {
            b"if" => If,
            b"else" => Else,
            b"true" => True,
            b"false" => False,
            b"let" => Let,
            b"fn" => Func,
            b"return" => Return,
            _ => Ident,
        };
        Token { kind, span }
    }

    fn number(&mut self) -> Token {
        self.eat_while(|c| c.is_ascii_digit());
        self.token(Number)
    }

    fn token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            span: self.span(),
        }
    }

    fn eat(&mut self, c: u8) -> bool {
        if self.peek_char() == c {
            self.advance();
            true
        } else {
            false
        }
    }

    fn eat_while<F>(&mut self, f: F)
    where
        F: Fn(u8) -> bool,
    {
        while f(self.peek_char()) {
            self.advance();
        }
    }

    fn span(&self) -> Span {
        Span::new(self.start, self.pos)
    }

    fn eof(&mut self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek_char(&self) -> u8 {
        self.source.get(self.pos).copied().unwrap_or_default()
    }

    fn next_char(&mut self) -> u8 {
        let c = self.peek_char();
        self.advance();
        c
    }

    fn advance(&mut self) {
        if !self.eof() {
            self.pos += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn dummy() -> Self {
        Self {
            kind: Eof,
            span: Span::dummy(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,

    Eq,
    EqEq,
    Not,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,

    And,
    AndAnd,
    Or,
    OrOr,

    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    LArrow,
    Colon,
    Semicolon,

    If,
    Else,
    True,
    False,
    Let,
    Func,
    Return,
    Ident,

    Number,

    Eof,
}

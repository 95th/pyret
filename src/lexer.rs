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
                b'-' => self.token(Minus),
                b'*' => self.token(Star),
                b'/' => self.token(Slash),
                c if c.is_ascii_digit() => self.number(),
                c if c.is_ascii_whitespace() => continue,
                c => panic!("Unexpected character {}", c),
            };
            return t;
        }

        self.token(Eof)
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

    fn eat_while<F>(&mut self, f: F)
    where
        F: Fn(u8) -> bool,
    {
        if f(self.peek_char()) {
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

    Number,

    Eof,
}

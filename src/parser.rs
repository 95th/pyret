use crate::{
    ast,
    lexer::{Lexer, Token, TokenKind},
};

pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    token: Token,
    prev: Token,
}

macro_rules! eat {
    ($self:ident, $( $pattern:pat )|+) => {
        match $self.token.kind {
            $( $pattern )|+ => {
                $self.advance();
                true
            },
            _ => false,
        }
    }
}

#[allow(unused)]
macro_rules! consume {
    ($self:ident, $( $pattern:pat )|+, $msg:literal) => {
        if eat!($self, $( $pattern )|+) {
            $self.prev.clone()
        } else {
            panic!($msg)
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        Self {
            source,
            lexer,
            token,
            prev: Token::dummy(),
        }
    }

    pub fn parse_expr(&mut self) -> ast::Expr {
        self.addition()
    }

    fn addition(&mut self) -> ast::Expr {
        let mut left = self.multiplication();

        while eat!(self, TokenKind::Plus | TokenKind::Minus) {
            let op = match self.prev.kind {
                TokenKind::Plus => ast::BinOp::Add,
                TokenKind::Minus => ast::BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.multiplication();
            let span = left.span.to(right.span);
            left = ast::Expr {
                kind: ast::ExprKind::Binary(op, Box::new(left), Box::new(right)),
                span,
            }
        }

        left
    }

    fn multiplication(&mut self) -> ast::Expr {
        let mut left = self.unary();

        while eat!(self, TokenKind::Star | TokenKind::Slash) {
            let op = match self.prev.kind {
                TokenKind::Star => ast::BinOp::Mul,
                TokenKind::Slash => ast::BinOp::Div,
                _ => unreachable!(),
            };
            let right = self.unary();
            let span = left.span.to(right.span);
            left = ast::Expr {
                kind: ast::ExprKind::Binary(op, Box::new(left), Box::new(right)),
                span,
            }
        }

        left
    }

    fn unary(&mut self) -> ast::Expr {
        if self.eat(TokenKind::Number) {
            let span = self.prev.span;
            let val = self.source[span.lo..span.hi].parse().unwrap();
            ast::Expr {
                kind: ast::ExprKind::Number(val),
                span,
            }
        } else {
            todo!()
        }
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.token.kind == kind
    }

    fn advance(&mut self) {
        self.prev = std::mem::replace(&mut self.token, self.lexer.next_token());
    }

    #[allow(unused)]
    fn eof(&self) -> bool {
        self.token.kind == TokenKind::Eof
    }
}

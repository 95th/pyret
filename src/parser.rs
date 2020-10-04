use crate::{
    ast::Block,
    ast::Literal,
    ast::UnOp,
    ast::{BinOp, Expr, ExprKind, Stmt},
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

    pub fn parse_stmt(&mut self) -> Stmt {
        todo!()
    }

    pub fn parse_expr(&mut self) -> Expr {
        if self.eat(TokenKind::If) {
            self.if_expr()
        } else {
            self.equality()
        }
    }

    fn if_expr(&mut self) -> Expr {
        let lo = self.prev.span;
        let cond = self.parse_expr();
        consume!(self, TokenKind::OpenBrace, "Expect '{' after if condition");
        let then_branch = self.parse_expr();
        consume!(self, TokenKind::CloseBrace, "Expect '}' after then branch");

        consume!(self, TokenKind::Else, "Expect 'else' after if condition");

        consume!(self, TokenKind::OpenBrace, "Expect '{' after else");
        let else_branch = self.parse_expr();
        consume!(self, TokenKind::CloseBrace, "Expect '}' after else branch");

        let span = lo.to(self.prev.span);
        Expr {
            kind: ExprKind::If(Box::new(cond), Box::new(then_branch), Box::new(else_branch)),
            span,
        }
    }

    fn equality(&mut self) -> Expr {
        let mut left = self.comparison();

        while eat!(self, TokenKind::Ne | TokenKind::EqEq) {
            let op = match self.prev.kind {
                TokenKind::Ne => BinOp::Ne,
                TokenKind::EqEq => BinOp::Eq,
                _ => unreachable!(),
            };
            let right = self.comparison();
            let span = left.span.to(right.span);
            left = Expr {
                kind: ExprKind::Binary(op, Box::new(left), Box::new(right)),
                span,
            }
        }

        left
    }

    fn comparison(&mut self) -> Expr {
        let mut left = self.addition();

        while eat!(
            self,
            TokenKind::Ge | TokenKind::Gt | TokenKind::Le | TokenKind::Lt
        ) {
            let op = match self.prev.kind {
                TokenKind::Ge => BinOp::Ge,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Le => BinOp::Le,
                TokenKind::Lt => BinOp::Lt,
                _ => unreachable!(),
            };
            let right = self.addition();
            let span = left.span.to(right.span);
            left = Expr {
                kind: ExprKind::Binary(op, Box::new(left), Box::new(right)),
                span,
            }
        }

        left
    }

    fn addition(&mut self) -> Expr {
        let mut left = self.multiplication();

        while eat!(self, TokenKind::Plus | TokenKind::Minus) {
            let op = match self.prev.kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.multiplication();
            let span = left.span.to(right.span);
            left = Expr {
                kind: ExprKind::Binary(op, Box::new(left), Box::new(right)),
                span,
            }
        }

        left
    }

    fn multiplication(&mut self) -> Expr {
        let mut left = self.unary();

        while eat!(self, TokenKind::Star | TokenKind::Slash) {
            let op = match self.prev.kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => unreachable!(),
            };
            let right = self.unary();
            let span = left.span.to(right.span);
            left = Expr {
                kind: ExprKind::Binary(op, Box::new(left), Box::new(right)),
                span,
            }
        }

        left
    }

    fn unary(&mut self) -> Expr {
        if self.eat(TokenKind::Minus) {
            let lo = self.prev.span;
            let expr = self.primary();
            let span = lo.to(expr.span);
            Expr {
                kind: ExprKind::Unary(UnOp::Neg, Box::new(expr)),
                span,
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.eat(TokenKind::Number) {
            let span = self.prev.span;
            let val = self.source[span.lo..span.hi].parse().unwrap();
            Expr {
                kind: ExprKind::Literal(Literal::Num(val)),
                span,
            }
        } else if self.eat(TokenKind::OpenParen) {
            let lo = self.prev.span;
            let expr = self.parse_expr();
            consume!(
                self,
                TokenKind::CloseParen,
                "Expected ')' after grouping expression"
            );
            let span = lo.to(self.prev.span);
            Expr {
                kind: ExprKind::Grouping(Box::new(expr)),
                span,
            }
        } else if eat!(self, TokenKind::True | TokenKind::False) {
            let val = match self.prev.kind {
                TokenKind::True => true,
                TokenKind::False => false,
                _ => unreachable!(),
            };
            Expr {
                kind: ExprKind::Literal(Literal::Bool(val)),
                span: self.prev.span,
            }
        } else if self.eat(TokenKind::OpenBrace) {
            let lo = self.prev.span;
            let block = self.block();
            let span = lo.to(self.prev.span);
            Expr {
                kind: ExprKind::Block(block),
                span,
            }
        } else {
            panic!("Unexpected token: {:?}", self.token)
        }
    }

    fn block(&mut self) -> Block {
        let mut stmts = vec![];

        while !self.eof() && !self.check(TokenKind::CloseBrace) {
            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }

        consume!(self, TokenKind::CloseBrace, "Expect '}' after block");
        Block { stmts }
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

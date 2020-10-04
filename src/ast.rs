use crate::{span::Span, symbol::Symbol};

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ExprKind {
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Block),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Ne,
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
}

#[derive(Debug)]
pub enum UnOp {
    Neg,
}

#[derive(Debug)]
pub enum Literal {
    Bool(bool),
    Num(i32),
}

#[derive(Debug)]
pub enum StmtKind {
    Let {
        ident: Symbol,
        ty: Symbol,
        init: Option<Expr>,
    },
    Assign {
        ident: Symbol,
        value: Expr,
    },
    Expr(Expr),
    ExprWithoutSemi(Expr),
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

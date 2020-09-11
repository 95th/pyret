use crate::ast::{BinOp, Expr, ExprKind, UnOp};

pub struct Eval {}

impl Eval {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> i32 {
        match &expr.kind {
            ExprKind::Number(n) => *n,
            ExprKind::Unary(op, e) => match op {
                UnOp::Neg => -1 * self.eval_expr(e),
            },
            ExprKind::Binary(op, l, r) => match op {
                BinOp::Add => self.eval_expr(l) + self.eval_expr(r),
                BinOp::Sub => self.eval_expr(l) - self.eval_expr(r),
                BinOp::Mul => self.eval_expr(l) * self.eval_expr(r),
                BinOp::Div => self.eval_expr(l) / self.eval_expr(r),
            },
            ExprKind::Grouping(e) => self.eval_expr(e),
        }
    }
}

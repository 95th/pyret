use crate::ast::{BinOp, Expr, ExprKind, Literal, UnOp};

pub struct Eval {}

#[derive(Debug)]
pub enum Value {
    Num(i32),
    Bool(bool),
}

macro_rules! eval_equality {
    ($self:ident, $l:expr, $r:expr, $op:tt) => {
        match ($self.eval_expr($l), $self.eval_expr($r)) {
            (Value::Bool(l), Value::Bool(r)) => Value::Bool(l $op r),
            (Value::Num(l), Value::Num(r)) => Value::Bool(l $op r),
            _ => panic!("Both values should be either Bool or Number"),
        }
    };
}

impl Eval {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Value {
        match &expr.kind {
            ExprKind::Literal(lit) => match lit {
                Literal::Bool(x) => Value::Bool(*x),
                Literal::Num(x) => Value::Num(*x),
            },
            ExprKind::Unary(op, e) => match op {
                UnOp::Neg => match self.eval_expr(e) {
                    Value::Num(n) => Value::Num(n * -1),
                    _ => panic!("Can negate only numbers"),
                },
            },
            ExprKind::Binary(op, l, r) => match op {
                BinOp::Add => self.eval_arith(l, r, |l, r| l + r),
                BinOp::Sub => self.eval_arith(l, r, |l, r| l - r),
                BinOp::Mul => self.eval_arith(l, r, |l, r| l * r),
                BinOp::Div => self.eval_arith(l, r, |l, r| l / r),

                BinOp::Ne => eval_equality!(self, l, r, !=),
                BinOp::Eq => eval_equality!(self, l, r, ==),

                BinOp::Ge => self.eval_bool(l, r, |l, r| l >= r),
                BinOp::Gt => self.eval_bool(l, r, |l, r| l > r),
                BinOp::Le => self.eval_bool(l, r, |l, r| l <= r),
                BinOp::Lt => self.eval_bool(l, r, |l, r| l < r),
            },
            ExprKind::Grouping(e) => self.eval_expr(e),
        }
    }

    fn eval_arith<F>(&mut self, l: &Expr, r: &Expr, f: F) -> Value
    where
        F: Fn(i32, i32) -> i32,
    {
        match (self.eval_expr(l), self.eval_expr(r)) {
            (Value::Num(l), Value::Num(r)) => Value::Num(f(l, r)),
            _ => panic!("Value not a number"),
        }
    }

    fn eval_bool<F>(&mut self, l: &Expr, r: &Expr, f: F) -> Value
    where
        F: Fn(i32, i32) -> bool,
    {
        match (self.eval_expr(l), self.eval_expr(r)) {
            (Value::Num(l), Value::Num(r)) => Value::Bool(f(l, r)),
            _ => panic!("Value not a number"),
        }
    }
}

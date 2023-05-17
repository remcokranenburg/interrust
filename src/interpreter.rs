use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;

use crate::parser::Value;
use crate::parser::Expr;

impl Add for Value {
    type Output = Value;

    fn add(self, other: Self) -> Self {
        match self {
            Value::I32(s) => {
                match other {
                    Value::I32(o) => Value::I32(s + o)
                }
            }
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Self) -> Self {
        match self {
            Value::I32(s) => {
                match other {
                    Value::I32(o) => Value::I32(s - o)
                }
            }
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Self) -> Self {
        match self {
            Value::I32(s) => {
                match other {
                    Value::I32(o) => Value::I32(s * o)
                }
            }
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Self) -> Self {
        match self {
            Value::I32(s) => {
                match other {
                    Value::I32(o) => Value::I32(s / o)
                }
            }
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self {
        match self {
            Value::I32(s) => Value::I32(-s)
        }
    }
}

pub fn eval(expr: Expr) -> Value {
    match expr {
        Expr::Value(v) => v,
        Expr::Add(a, b) => eval(*a) + eval(*b),
        Expr::Sub(a, b) => eval(*a) - eval(*b),
        Expr::Mul(a, b) => eval(*a) * eval(*b),
        Expr::Div(a, b) => eval(*a) / eval(*b),
        Expr::Neg(x) => -eval(*x),
    }
}

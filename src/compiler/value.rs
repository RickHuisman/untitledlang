use crate::compiler::object::{Closure, Function};
use crate::vm::obj::Gc;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    String(String),
    Closure(Gc<Closure>),
    Function(Gc<Function>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Nil => write!(f, "nil"),
            Value::Closure(clos) => write!(f, "Closure({:?})", clos),
            Value::Function(fun) => write!(f, "Function({})", **fun),
        }
    }
}

impl From<&Value> for bool {
    fn from(value: &Value) -> Self {
        match value {
            Value::Bool(false) | Value::Nil => false,
            _ => true,
        }
    }
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        Value::Bool(self)
    }
}

// TODO: Return errors, not panic.
impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(b), Value::Number(a)) => Value::Number(b + a),
            _ => panic!("Operand must be a number."),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(b), Value::Number(a)) => Value::Number(b - a),
            _ => panic!("Operand must be a number."),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(b), Value::Number(a)) => Value::Number(b * a),
            _ => panic!("Operand must be a number."),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Number(b), Value::Number(a)) => Value::Number(b / a),
            _ => panic!("Operand must be a number."),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(a) => Value::Number(-a),
            _ => panic!("Operand must be a number."),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        let b = self;
        let a = other;

        match (b, a) {
            (Value::Number(b), Value::Number(a)) => b == a,
            (Value::Bool(b), Value::Bool(a)) => b == a,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let b = self;
        let a = other;

        match (b, a) {
            (Value::Number(b), Value::Number(a)) => b.partial_cmp(a),
            (Value::Bool(b), Value::Bool(a)) => b.partial_cmp(a),
            _ => None,
        }
    }
}

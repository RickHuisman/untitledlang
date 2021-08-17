use std::cmp::Ordering;
use std::ops::{Div, Neg, Mul, Sub, Add};
use std::fmt;
use std::fmt::Formatter;
use crate::vm::obj::Gc;
use crate::compiler::object::{Closure, Function};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    True,
    False,
    Nil,
    Closure(Gc<Closure>),
    Function(Gc<Function>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::True => write!(f, "True"),
            Value::False => write!(f, "False"),
            Value::Nil => write!(f, "Nil"),
            Value::Closure(clos) => write!(f, "Closure({:?})", clos),
            Value::Function(fun) => write!(f, "Function({})", **fun),
        }
    }
}

impl From<&Value> for bool {
    fn from(value: &Value) -> Self {
        match value {
            Value::False | Value::Nil => false,
            _ => true,
        }
    }
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        if self {
            Value::True
        } else {
            Value::False
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                Value::Number(b + a)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                Value::Number(b - a)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                Value::Number(b * a)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                Value::Number(b / a)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(a) => Value::Number(-a),
            _ => todo!(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                b == a
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Value::Number(b) = self {
            if let Value::Number(a) = other {
                b.partial_cmp(a)
            } else {
                panic!("Operand must be a number.");
            }
        } else {
            panic!("Operand must be a number.");
        }
    }
}

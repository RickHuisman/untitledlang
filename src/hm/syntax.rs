use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Syntax {
    Lambda { v: String, body: Box<Syntax> },
    Identifier { name: String },
    Apply { func: Box<Syntax>, arg: Box<Syntax> },
    Let { v: String, defn: Box<Syntax>, body: Box<Syntax> },
    Letrec { v: String, defn: Box<Syntax>, body: Box<Syntax> },
}

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Syntax::*;
        match self {
            &Lambda { ref v, ref body } => {
                write!(f, "(fn {v} => {body})", v = v, body = body)
            }
            &Identifier { ref name } => {
                write!(f, "{}", name)
            }
            &Apply { ref func, ref arg } => {
                write!(f, "({func} {arg})", func = func, arg = arg)
            }
            &Let { ref v, ref defn, ref body } => {
                write!(f, "(let {v} = {defn} in {body})", v = v, defn = defn, body = body)
            }
            &Letrec { ref v, ref defn, ref body } => {
                write!(f, "(letrec {v} = {defn} in {body})", v = v, defn = defn, body = body)
            }
        }
    }
}

pub fn lambda(v: &str, body: Syntax) -> Syntax {
    Syntax::Lambda {
        v: v.to_string(),
        body: Box::new(body),
    }
}

pub fn apply(func: Syntax, arg: Syntax) -> Syntax {
    Syntax::Apply {
        func: Box::new(func),
        arg: Box::new(arg),
    }
}

pub fn let_(v: &str, defn: Syntax, body: Syntax) -> Syntax {
    Syntax::Let {
        v: v.to_string(),
        defn: Box::new(defn),
        body: Box::new(body),
    }
}

pub fn letrec(v: &str, defn: Syntax, body: Syntax) -> Syntax {
    Syntax::Letrec {
        v: v.to_string(),
        defn: Box::new(defn),
        body: Box::new(body),
    }
}

pub fn ident(name: &str) -> Syntax {
    Syntax::Identifier {
        name: name.to_string(),
    }
}

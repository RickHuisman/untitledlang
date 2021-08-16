use crate::vm::error::RuntimeError;
use std::collections::HashMap;
use crate::compiler::value::Value;
use crate::lexer::lexer::lex;
use crate::parser::parser::parse;
use crate::compiler::compiler::compile;

pub type RunResult<T> = Result<T, RuntimeError>;

pub fn interpret(source: &str) {
    let mut tokens = match lex(source) {
        Ok(t) => t,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let ast = match parse(&mut tokens) {
        Ok(a) => a,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let fun = compile(ast);
}

struct VM {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::with_capacity(256),
            globals: HashMap::new(),
        }
    }
}
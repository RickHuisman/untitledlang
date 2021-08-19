use crate::compiler::chunk::Chunk;
use crate::vm::obj::Gc;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum FunctionType {
    Closure,
    Function,
    Script,
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub fun: Gc<Function>,
}

impl Closure {
    pub fn new(fun: Gc<Function>) -> Closure {
        Closure { fun }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    chunk: Chunk,
    arity: u8,
}

impl Function {
    pub fn new() -> Self {
        Function {
            name: String::new(),
            chunk: Chunk::new(),
            arity: 0,
        }
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }

    pub fn arity(&self) -> &u8 {
        &self.arity
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    pub fn arity_mut(&mut self) -> &mut u8 {
        &mut self.arity
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO
        // if let Some(name) = self.chunk. {
        write!(f, "<fn {}>", self.name)
        // } else {
        //     write!(f, "<script>")
        // }
    }
}

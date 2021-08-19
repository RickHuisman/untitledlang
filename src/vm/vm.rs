use crate::vm::error::RuntimeError;
use std::collections::HashMap;
use crate::compiler::value::Value;
use crate::lexer::lexer::lex;
use crate::parser::parser::parse;
use crate::compiler::compiler::compile;
use crate::vm::frame::CallFrame;
use crate::compiler::chunk::Chunk;
use crate::vm::obj::Gc;
use crate::compiler::object::{Closure, Function};
use std::io::{Write, Stdout, stdout};

pub type Result<T> = std::result::Result<T, RuntimeError>;

pub fn interpret(source: &str) {
    let fun = compile(source).unwrap(); // TODO: Unwrap
    println!("{}", fun.chunk());

    let mut vm = VM::new();
    vm.interpret(fun);
}

pub struct VM<W: Write> {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    globals: HashMap<String, Value>,
    stdout: W, // TODO: Rename?
}

impl VM<Stdout> {
    pub fn new() -> Self {
        VM::with_stdout(stdout())
    }
}

impl<W: Write> VM<W> {
    pub fn with_stdout(stdout: W) -> Self {
        VM {
            stack: Vec::with_capacity(u8::MAX as usize),
            frames: Vec::with_capacity(u8::MAX as usize),
            globals: HashMap::new(),
            stdout,
        }
    }

    pub fn interpret(&mut self, fun: Function) -> Result<()> {
        let closure = self.alloc(Closure::new(Gc::new(fun)).clone());
        self.push(Value::Closure(closure));
        self.call_value(0);

        self.run()
    }

    pub fn call_value(&mut self, arity: u8) {
        let frame_start = self.stack.len() - (arity + 1) as usize;
        let callee = self.stack[frame_start].clone();

        match callee {
            Value::Closure(c) => self.call(c, arity),
            _ => panic!("Can only call functions"), // TODO Error
        }
    }

    fn call(&mut self, closure: Gc<Closure>, arity: u8) {
        if arity != *closure.fun.arity() {
            panic!( // TODO Error
                    "Expected {} arguments but got {}.",
                    closure.fun.arity(),
                    arity
            );
        }

        let last = self.stack.len();
        let frame_start = last - (arity + 1) as usize;

        self.frames.push(CallFrame::new(closure, frame_start));
    }

    pub(crate) fn read_string(&mut self) -> Result<String> {
        // TODO: Clean up.
        let foo = self.read_constant()?.clone();
        match foo {
            Value::String(s) => Ok(s),
            _ => Err(RuntimeError::ArgumentTypes),
        }
    }

    pub fn read_constant(&mut self) -> Result<&Value> {
        let constant_index = self.read_byte()?;
        Ok(self.current_chunk()?.read_constant(constant_index.into()))
    }

    pub fn read_byte(&mut self) -> Result<u8> {
        let index = *self.frame()?.ip();
        let byte = self.current_chunk_mut()?.code()[index];
        *self.frame_mut()?.ip_mut() += 1;
        Ok(byte)
    }

    fn read_short(&mut self) -> Result<u16> {
        *self.frame_mut()?.ip_mut() += 2;

        let lo_index = self.frame()?.ip() - 2;
        let hi_index = self.frame()?.ip() - 1;

        let lo = self.current_chunk_mut()?.code()[lo_index] as u16;
        let hi = self.current_chunk_mut()?.code()[hi_index] as u16;
        Ok((lo << 8) | hi)
    }

    pub fn is_at_end(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn peek(&mut self) -> Result<&Value> {
        self.stack.last().ok_or(RuntimeError::StackEmpty)
    }

    fn peek_offset(&mut self, offset: usize) -> Result<&Value> {
        let index = self.stack.len() - 1 - offset; // TODO Error
        Ok(&self.stack[index as usize])
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or(RuntimeError::StackEmpty)
    }

    pub fn stack(&self) -> &Vec<Value> {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Vec<Value> {
        &mut self.stack
    }

    pub fn frame(&self) -> Result<&CallFrame> {
        self.frames.last().ok_or(RuntimeError::FrameEmpty)
    }

    fn frame_mut(&mut self) -> Result<&mut CallFrame> {
        self.frames.last_mut().ok_or(RuntimeError::FrameEmpty)
    }

    pub fn frames_mut(&mut self) -> &mut Vec<CallFrame> {
        &mut self.frames
    }

    pub fn globals(&self) -> &HashMap<String, Value> {
        &self.globals
    }

    pub fn globals_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self.globals
    }

    pub fn stdout_mut(&mut self) -> &mut W {
        &mut self.stdout
    }

    fn current_chunk(&self) -> Result<&Chunk> {
        Ok(&self.frame()?.closure().fun.chunk())
    }

    fn current_chunk_mut(&mut self) -> Result<&mut Chunk> {
        Ok(self.frame_mut()?.closure_mut().fun.chunk_mut())
    }
}
use crate::compiler::chunk::Chunk;
use crate::compiler::object::{Closure, Function};
use crate::compiler::value::Value;
use crate::vm::error::{RunResult, RuntimeError};
use crate::vm::frame::CallFrame;
use crate::vm::obj::Gc;
use std::collections::HashMap;
use std::io::{stdout, Stdout, Write};

pub struct VM<W: Write> {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    globals: HashMap<String, Value>,
    stdout: W,
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

    pub fn interpret(&mut self, fun: Function) -> RunResult<()> {
        let closure = self.alloc(Closure::new(Gc::new(fun)).clone());
        self.push(Value::Closure(closure));
        self.call_value(0);

        self.run()
    }

    pub fn call_value(&mut self, arity: u8) -> RunResult<()> {
        let frame_start = self.stack.len() - (arity + 1) as usize;
        let callee = self.stack[frame_start].clone();

        match callee {
            Value::Closure(c) => self.call(c, arity),
            _ => return Err(RuntimeError::InvalidCallee),
        };

        Ok(())
    }

    fn call(&mut self, closure: Gc<Closure>, arity: u8) -> RunResult<()> {
        if arity != *closure.fun.arity() {
            return Err(RuntimeError::IncorrectArity);
        }

        let last = self.stack.len();
        let frame_start = last - (arity + 1) as usize;

        self.frames.push(CallFrame::new(closure, frame_start));
        Ok(())
    }

    pub fn read_string(&mut self) -> RunResult<String> {
        match self.read_constant()? {
            Value::String(s) => Ok(s.clone()),
            _ => Err(RuntimeError::ArgumentTypes),
        }
    }

    pub fn read_constant(&mut self) -> RunResult<&Value> {
        let constant_index = self.read_byte()?;
        Ok(self.current_chunk()?.read_constant(constant_index as usize))
    }

    pub fn read_byte(&mut self) -> RunResult<u8> {
        let index = *self.frame()?.ip();
        let byte = self.current_chunk()?.code()[index];
        *self.frame_mut()?.ip_mut() += 1;
        Ok(byte)
    }

    fn read_short(&mut self) -> RunResult<u16> {
        *self.frame_mut()?.ip_mut() += 2;

        let lo_index = self.frame()?.ip() - 2;
        let hi_index = self.frame()?.ip() - 1;

        let lo = self.current_chunk()?.code()[lo_index] as u16;
        let hi = self.current_chunk()?.code()[hi_index] as u16;
        Ok((lo << 8) | hi)
    }

    pub fn is_at_end(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn peek(&mut self) -> RunResult<&Value> {
        self.stack.last().ok_or(RuntimeError::StackEmpty)
    }

    pub fn pop(&mut self) -> RunResult<Value> {
        self.stack.pop().ok_or(RuntimeError::StackEmpty)
    }

    pub fn stack(&self) -> &Vec<Value> {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Vec<Value> {
        &mut self.stack
    }

    pub fn frame(&self) -> RunResult<&CallFrame> {
        self.frames.last().ok_or(RuntimeError::FrameEmpty)
    }

    fn frame_mut(&mut self) -> RunResult<&mut CallFrame> {
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

    fn current_chunk(&self) -> RunResult<&Chunk> {
        Ok(&self.frame()?.closure().fun.chunk())
    }

    fn current_chunk_mut(&mut self) -> RunResult<&mut Chunk> {
        Ok(self.frame_mut()?.closure_mut().fun.chunk_mut())
    }
}

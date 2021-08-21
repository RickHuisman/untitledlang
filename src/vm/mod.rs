use crate::compiler::compile;
use crate::vm::error::RunResult;
use crate::vm::vm::VM;
use std::io::Write;

mod error;
mod frame;
mod gc;
pub mod obj;
pub mod opcode;
mod run;
mod vm;

pub fn interpret(source: &str) {
    // TODO: Report errors.
    let fun = compile(source).unwrap();

    let mut vm = VM::new();
    vm.interpret(fun).unwrap();
}

pub fn interpret_with_stdout<W: Write>(source: &str, stdout: W) -> RunResult<()> {
    // TODO: Report errors.
    let fun = compile(source).unwrap();
    let mut vm = VM::with_stdout(stdout);
    vm.interpret(fun)
}

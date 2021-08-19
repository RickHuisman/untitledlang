use crate::compiler::compile;
use crate::vm::vm::VM;

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
    println!("{}", fun.chunk());

    let mut vm = VM::new();
    vm.interpret(fun).unwrap();
}

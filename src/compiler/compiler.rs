use crate::compiler::instance::CompilerInstance;
use crate::parser::ast::ModuleAst;
use crate::compiler::error::CompilerError;
use crate::compiler::object::{Function, FunctionType};
use crate::vm::opcode::Opcode;
use crate::compiler::expr_compiler::compile_expr;
use crate::compiler::chunk::Chunk;
use crate::compiler::value::Value;

type Result<T> = std::result::Result<T, CompilerError>;

pub fn compile(ast: ModuleAst) -> Result<Function> {
    let mut compiler = Compiler::new();

    for expr in ast {
        compile_expr(&mut compiler, expr);
    }

    Ok(compiler.end_compiler())
}

pub struct Compiler {
    current: CompilerInstance,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            current: CompilerInstance::new(FunctionType::Script),
        }
    }

    pub fn end_compiler(&mut self) -> Function {
        // TODO: Clones???
        self.emit_return();
        let fun_copy = self.current.function().clone();

        if let Some(enclosing) = *self.current.enclosing().clone() {
            self.current = enclosing;
        }

        fun_copy
    }

    pub fn emit_return(&mut self) {
        // self.emit(Opcode::Nil); // TODO: Return Nil???
        self.emit(Opcode::Return);
    }

    pub fn emit_constant(&mut self, value: Value) {
        let constant = self.current_chunk().add_constant(value);
        self.emit(Opcode::Constant);
        self.emit_byte(constant);
    }

    pub fn emit(&mut self, opcode: Opcode) {
        self.current_chunk().write(opcode, 123); // TODO Line
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.current_chunk().write_byte(byte);
    }

    pub fn current_chunk(&mut self) -> &mut Chunk {
        self.current.function_mut().chunk_mut()
    }
}
use crate::compiler::chunk::Chunk;
use crate::compiler::error::{CompileResult, CompilerError};
use crate::compiler::instance::CompilerInstance;
use crate::compiler::object::{Function, FunctionType};
use crate::compiler::value::Value;
use crate::parser::ast::Identifier;
use crate::vm::opcode::{Opcode, StackIndex};

pub struct Compiler {
    current: CompilerInstance,
    errors: Vec<CompilerError>,
}

impl Compiler {
    pub(crate) fn new() -> Self {
        Compiler {
            current: CompilerInstance::new(FunctionType::Script),
            errors: vec![],
        }
    }

    pub fn declare_variable(&mut self, ident: &Identifier) {
        if self.is_scoped() {
            if self.contains_local_in_current_scope(&ident) {
                self.add_error(CompilerError::LocalAlreadyDefined);
            }

            self.add_local(&ident);
        }
    }

    pub fn define_variable(&mut self, ident: &Identifier) {
        if self.is_scoped() {
            self.mark_local_initialized();
            return;
        }

        self.emit(Opcode::DefineGlobal);
        let constant_id = self
            .current_chunk()
            .add_constant(Value::String(ident.to_string()));
        self.emit_byte(constant_id);
    }

    pub fn resolve_local(&mut self, name: &str) -> Option<StackIndex> {
        match self.current().resolve_local(name) {
            Err(error) => {
                self.add_error(error);
                None
            }
            Ok(local) => local,
        }
    }

    pub fn add_local(&mut self, ident: &Identifier) {
        self.current.locals_mut().insert(ident);
    }

    pub fn contains_local_in_current_scope(&self, name: &str) -> bool {
        self.current.locals().get_at_current_depth(name).is_some()
    }

    fn mark_local_initialized(&mut self) {
        if !self.is_scoped() {
            return;
        }

        self.current_mut().locals_mut().mark_initialized();
    }

    pub fn begin_scope(&mut self) {
        self.current_mut().locals_mut().begin_scope();
    }

    pub fn end_scope(&mut self) {
        for _ in self.current_mut().locals_mut().end_scope().iter().rev() {
            self.emit(Opcode::Pop);
        }
    }

    pub fn is_scoped(&self) -> bool {
        self.current().locals().scope_depth() > 0
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

    pub fn emit_string(&mut self, s: &str) {
        self.emit_constant(Value::String(s.to_string()));
    }

    pub fn add_error(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    pub fn emit(&mut self, opcode: Opcode) {
        self.current_chunk().write(opcode, 123); // TODO Line
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.current_chunk().write_byte(byte);
    }

    pub fn current(&self) -> &CompilerInstance {
        &self.current
    }

    pub fn current_mut(&mut self) -> &mut CompilerInstance {
        &mut self.current
    }

    pub fn current_chunk(&mut self) -> &mut Chunk {
        self.current.function_mut().chunk_mut()
    }
}

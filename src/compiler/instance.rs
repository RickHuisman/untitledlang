use crate::compiler::error::{CompileResult, CompilerError};
use crate::compiler::local::Locals;
use crate::compiler::object::{Function, FunctionType};
use crate::vm::opcode::StackIndex;

#[derive(Clone)]
pub struct CompilerInstance {
    function: Function,
    function_type: FunctionType,
    locals: Locals,
    enclosing: Box<Option<CompilerInstance>>,
}

impl CompilerInstance {
    pub fn new(function_type: FunctionType) -> Self {
        CompilerInstance {
            function: Function::new(),
            function_type,
            locals: Locals::new(),
            enclosing: Box::new(None),
        }
        // compiler.locals.insert(&String::new()); // TODO: Is this necessary?
    }

    pub fn resolve_local(&self, name: &str) -> CompileResult<Option<StackIndex>> {
        if let Some(local) = self.locals.get(name) {
            return if !local.initialized() {
                Err(CompilerError::LocalNotInitialized)
            } else {
                Ok(Some(local.slot()))
            };
        }

        Ok(None)
    }

    pub fn function(&self) -> &Function {
        &self.function
    }

    pub fn function_mut(&mut self) -> &mut Function {
        &mut self.function
    }

    pub fn locals(&self) -> &Locals {
        &self.locals
    }

    pub fn locals_mut(&mut self) -> &mut Locals {
        &mut self.locals
    }

    pub fn enclosing(&self) -> &Box<Option<CompilerInstance>> {
        &self.enclosing
    }
}

use crate::compiler::object::{FunctionType, Function};
use crate::compiler::local::{Local, Locals};
use crate::vm::opcode::StackIndex;
use crate::compiler::error::{Result, CompileError};

#[derive(Clone)]
pub struct CompilerInstance {
    function: Function,
    function_type: FunctionType,
    locals: Locals,
    // scope_depth: isize,
    enclosing: Box<Option<CompilerInstance>>,
}

impl CompilerInstance {
    pub fn new(function_type: FunctionType) -> Self {
        let mut compiler = CompilerInstance {
            function: Function::new(),
            function_type,
            locals: Locals::new(),
            // scope_depth: 0,
            enclosing: Box::new(None),
        };
        compiler.locals.insert(&String::new()); // TODO: Is this necessary?

        compiler
    }

    pub fn resolve_local(&self, name: &String) -> Result<Option<StackIndex>> {
        if let Some(local) = self.locals.get(name) {
            if !local.initialized() {
                Err(CompileError::LocalNotInitialized)
            } else {
                Ok(Some(local.slot()))
            }
        } else {
            Ok(None)
        }

        // // TODO: Clean up?
        // for (i, local) in self.current.locals().iter().enumerate() {
        //     if *name == *local.name() {
        //         if *local.depth() == -1 {
        //             panic!(
        //                 "Can't read local variable {} in it's own initializer.",
        //                 name
        //             );
        //         }
        //
        //         // TODO: Return slot.
        //         return Some(i);
        //     }
        // }
        //
        // None
    }

    pub fn function(&self) -> &Function {
        &self.function
    }

    pub fn function_mut(&mut self) -> &mut Function {
        &mut self.function
    }

    pub fn function_type(&self) -> &FunctionType {
        &self.function_type
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

    pub fn enclosing_mut(&mut self) -> &mut Box<Option<CompilerInstance>> {
        &mut self.enclosing
    }
}

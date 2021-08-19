use crate::compiler::instance::CompilerInstance;
use crate::parser::ast::{ModuleAst, Identifier};
use crate::compiler::error::{Result, CompileError};
use crate::compiler::object::{Function, FunctionType};
use crate::vm::opcode::{Opcode, StackIndex};
use crate::compiler::expr_compiler::compile_expr;
use crate::compiler::chunk::Chunk;
use crate::compiler::value::Value;
use crate::compiler::local::Local;
use crate::lexer::lexer::lex;
use crate::parser::parser::parse;

pub fn compile(source: &str) -> Result<Function> {
    // TODO: Report errors.
    let mut tokens = lex(source).unwrap();
    let ast = parse(&mut tokens).unwrap();

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

    pub fn declare_variable(&mut self, ident: &Identifier) -> Result<()> {
        // TODO: Use is_scoped.
        if self.is_scoped() {
            if self.contains_local_in_current_scope(&ident) {
                panic!("TEST"); // TODO
            }

            self.add_local(&ident);
            // TODO: self.mark_initialized();
        }
        // if *self.current.scope_depth() == 0 {
        //     return Ok(());
        // }


        // for local in self.current.locals() {
        //     if *local.depth() != -1 as isize &&
        //         local.depth() < &self.current.scope_depth() {
        //         break;
        //     }
        //
        //     if *ident == *local.name() {
        //         return Err(CompileError::LocalAlreadyDefined);
        //     }
        // }

        // self.add_local(ident);
        // self.mark_initialized();
        Ok(())
    }

    pub fn define_variable(&mut self, ident: &Identifier) {
        if self.is_scoped() {
            self.mark_initialized();
            return;
        }

        self.emit(Opcode::DefineGlobal);
        let constant_id = self
            .current_chunk()
            .add_constant(Value::String(ident.to_string()));
        self.emit_byte(constant_id);
    }

    pub fn add_local(&mut self, ident: &Identifier) {
        self.current.locals_mut().insert(ident);
    }

    pub fn contains_local_in_current_scope(&self, name: &str) -> bool {
        self.current
            .locals()
            .get_at_current_depth(name)
            .is_some()
    }

    fn mark_initialized(&mut self) {
        // TODO: Use !is_scoped().
        // TODO: Return early if not scoped
        // if *self.current.scope_depth() == 0 {
        //     return;
        // }

        self.current_mut().locals_mut().mark_initialized();

        // TODO: Remove
        // let index = &self.current.locals().len() - 1;
        // *self.current.locals_mut()[index].depth_mut() = *self.current.scope_depth();
    }

    pub fn begin_scope(&mut self) {
        self.current_mut().locals_mut().begin_scope();
    }

    pub fn end_scope(&mut self) {
        for local in self.current_mut().locals_mut().end_scope().iter().rev() {
            // TODO
            // if local.captured() {
            //     self.add_instruction(Instruction::CloseUpvalue);
            // } else {
            self.emit(Opcode::Pop);
            // }
        }
    }

    // pub fn end_scope(&mut self) {
    //     *self.current.scope_depth_mut() -= 1;
    //
    //     while self.current.locals().len() > 0
    //         && self.current.locals()[self.current.locals().len() - 1].depth()
    //         > self.current.scope_depth() {
    //         self.emit(Opcode::Pop);
    //         self.current.locals_mut().pop();
    //     }
    // }

    pub fn is_scoped(&mut self) -> bool {
        let c = self.current();
        c.locals().scope_depth() > 0
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
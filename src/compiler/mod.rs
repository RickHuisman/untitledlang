use crate::compiler::compiler::Compiler;
use crate::compiler::error::CompileResult;
use crate::compiler::expr_compiler::compile_expr;
use crate::compiler::object::Function;
use crate::lexer::lex;
use crate::parser::parse;

pub mod chunk;
pub mod compiler;
mod error;
mod expr_compiler;
mod instance;
mod local;
pub mod object;
pub mod value;

pub fn compile(source: &str) -> CompileResult<Function> {
    // TODO: Report errors.
    let mut tokens = lex(source).unwrap();
    let ast = parse(&mut tokens).unwrap();

    let mut compiler = Compiler::new();

    for expr in ast {
        compile_expr(&mut compiler, expr);
    }

    Ok(compiler.end_compiler())
}

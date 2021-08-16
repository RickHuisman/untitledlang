use crate::compiler::compiler::Compiler;
use crate::compiler::error::CompilerError;
use crate::parser::ast::Expr;

type Result<T> = std::result::Result<T, CompilerError>;

pub fn compile_expr(compiler: &mut Compiler, expr: Expr) {
    match expr {
        Expr::Grouping { .. } => {}
        Expr::Binary { .. } => {}
        Expr::Unary { .. } => {}
        Expr::LetAssign { .. } => {}
        Expr::LetGet { .. } => {}
        Expr::LetSet { .. } => {}
        Expr::Block { exprs } => {}
        Expr::Literal(l) => {}
    }
    println!("Hello");
}

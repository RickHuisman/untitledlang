use crate::compiler::compiler::Compiler;
use crate::compiler::error::CompilerError;
use crate::parser::ast::{Expr, BinaryOperator, LiteralExpr};
use crate::vm::opcode::Opcode;
use crate::compiler::value::Value;

type Result<T> = std::result::Result<T, CompilerError>;

pub fn compile_expr(compiler: &mut Compiler, expr: Expr) {
    match expr {
        Expr::Grouping { .. } => {}
        Expr::Binary { left, op, right } => {
            compile_binary(compiler, left, op, right)
        }
        Expr::Unary { .. } => {}
        Expr::LetAssign { .. } => {}
        Expr::LetGet { .. } => {}
        Expr::LetSet { .. } => {}
        Expr::Block { exprs } => {}
        Expr::Literal(l) => compile_literal(compiler, l),
    }
}

fn compile_binary(
    compiler: &mut Compiler,
    left: Box<Expr>,
    op: BinaryOperator,
    right: Box<Expr>,
) {
    compile_expr(compiler, *left);
    compile_expr(compiler, *right);

    match op {
        BinaryOperator::Add => compiler.emit(Opcode::Add),
        BinaryOperator::Subtract => compiler.emit(Opcode::Subtract),
        BinaryOperator::Multiply => compiler.emit(Opcode::Multiply),
        BinaryOperator::Divide => compiler.emit(Opcode::Divide),
        BinaryOperator::Equal => compiler.emit(Opcode::Equal),
        BinaryOperator::BangEqual => {
            compiler.emit(Opcode::Equal);
            compiler.emit(Opcode::Not);
        }
        BinaryOperator::GreaterThan => compiler.emit(Opcode::Greater),
        BinaryOperator::GreaterThanEqual => {
            compiler.emit(Opcode::Less);
            compiler.emit(Opcode::Not);
        }
        BinaryOperator::LessThan => compiler.emit(Opcode::Less),
        BinaryOperator::LessThanEqual => {
            compiler.emit(Opcode::Greater);
            compiler.emit(Opcode::Not);
        }
    }
}

fn compile_literal(compiler: &mut Compiler, literal: LiteralExpr) {
    match literal {
        LiteralExpr::Number(n) => compiler.emit_constant(Value::Number(n)),
        LiteralExpr::Nil => todo!(),
    }
}
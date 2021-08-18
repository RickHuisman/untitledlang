use crate::compiler::compiler::Compiler;
use crate::compiler::error::CompilerError;
use crate::parser::ast::{Expr, BinaryOperator, LiteralExpr, BlockDecl, UnaryOperator};
use crate::vm::opcode::Opcode;
use crate::compiler::value::Value;

type Result<T> = std::result::Result<T, CompilerError>;

pub fn compile_expr(c: &mut Compiler, expr: Expr) {
    match expr {
        Expr::Grouping { expr } => compile_expr(c, *expr),
        Expr::Binary { left, op, right } => {
            compile_binary(c, left, op, right)
        }
        Expr::Unary { op, expr } => compile_unary(c, op, expr),
        Expr::LetAssign { .. } => {}
        Expr::LetGet { .. } => {}
        Expr::LetSet { .. } => {}
        Expr::Fun { .. } => {}
        Expr::Block { block } => compile_block(c, block),
        Expr::Print { expr } => compile_print(c, expr),
        Expr::Literal(expr) => compile_literal(c, expr),
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

fn compile_unary(compiler: &mut Compiler, op: UnaryOperator, expr: Box<Expr>) {
    compile_expr(compiler, *expr);
    compiler.emit(Opcode::from(op));
}

fn compile_block(compiler: &mut Compiler, block: BlockDecl) {
    compiler.begin_scope();
    for expr in block {
        compile_expr(compiler, expr);
    }
    compiler.end_scope();
}

fn compile_print(compiler: &mut Compiler, expr: Box<Expr>) {
    compile_expr(compiler, *expr);
    compiler.emit(Opcode::Print);
}

fn compile_literal(compiler: &mut Compiler, literal: LiteralExpr) {
    match literal {
        LiteralExpr::Number(n) => compiler.emit_constant(Value::Number(n)),
        LiteralExpr::Nil => todo!(),
    }
}
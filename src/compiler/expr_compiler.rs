use crate::compiler::compiler::Compiler;
use crate::compiler::error::CompileError;
use crate::parser::ast::{Expr, BinaryOperator, LiteralExpr, BlockDecl, UnaryOperator, Identifier};
use crate::vm::opcode::Opcode;
use crate::compiler::value::Value;

type Result<T> = std::result::Result<T, CompileError>;

pub fn compile_expr(c: &mut Compiler, expr: Expr) {
    match expr {
        Expr::Grouping { expr } => compile_expr(c, *expr),
        Expr::Binary { left, op, right } => {
            compile_binary(c, left, op, right)
        }
        Expr::Unary { op, expr } => compile_unary(c, op, expr),
        Expr::LetAssign { ident, initializer } => {
            compile_let_assign(c, ident, initializer)
        }
        Expr::LetGet { ident } => compile_let_get(c, ident),
        Expr::LetSet { ident, expr } => compile_let_set(c, ident, expr),
        Expr::Fun { .. } => todo!(),
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

fn compile_let_assign(compiler: &mut Compiler, ident: Identifier, init: Box<Expr>) {
    compiler.declare_variable(&ident);

    // TODO: Check if initialized -> if not init with nil.
    // if let Some(expr) = expr {
    //     compile_expr(compiler, expr.as_ref());
    // } else {
    //     // compile_nil(compiler);
    // }

    // Compile initializer.
    compile_expr(compiler, *init);

    compiler.define_variable(&ident);

    // compile_expr(compiler, *init);
    //
    // if *compiler.current().scope_depth() > 0 {
    //     // Local variable
    //     compiler.declare_variable(ident);
    // } else {
    //     // Global variable
    //     compiler.define_variable(ident);
    // }
}

fn compile_let_get(compiler: &mut Compiler, ident: Identifier) {
    if let Ok(arg) = compiler.current().resolve_local(&ident) {
        if let Some(arg) = arg {
            // Local variable
            compiler.emit(Opcode::GetLocal);
            compiler.emit_byte(arg as u8);
        } else {
            // Global variable
            compiler.emit(Opcode::GetGlobal);
            let constant_id = compiler
                .current_chunk()
                .add_constant(Value::String(ident));
            compiler.emit_byte(constant_id);
        }
    }
}

fn compile_let_set(compiler: &mut Compiler, ident: Identifier, expr: Box<Expr>) {
    compile_expr(compiler, *expr);

    if let Ok(arg) = compiler.current().resolve_local(&ident) {
        if let Some(arg) = arg {
            // Local variable
            compiler.emit(Opcode::SetLocal);
            compiler.emit_byte(arg as u8);
        } else {
            // Global variable
            compiler.emit(Opcode::SetGlobal);
            let constant_id = compiler
                .current_chunk()
                .add_constant(Value::String(ident));
            compiler.emit_byte(constant_id);
        }
    }
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
        LiteralExpr::Nil => todo!(), // TODO: Compile nil literals.
    }
}
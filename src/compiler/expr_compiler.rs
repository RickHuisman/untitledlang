use crate::compiler::compiler::Compiler;
use crate::compiler::error::CompilerError;
use crate::compiler::instance::CompilerInstance;
use crate::compiler::object::FunctionType;
use crate::compiler::value::Value;
use crate::parser::ast::{
    BinaryOperator, BlockDecl, Expr, FunDecl, Identifier, LiteralExpr, UnaryOperator,
};
use crate::vm::obj::Gc;
use crate::vm::opcode::Opcode;

pub fn compile_expr(c: &mut Compiler, expr: Expr) {
    match expr {
        Expr::Grouping { expr } => compile_expr(c, *expr),
        Expr::Binary { left, op, right } => compile_binary(c, left, op, right),
        Expr::Unary { op, expr } => compile_unary(c, op, expr),
        Expr::LetAssign { ident, initializer } => compile_let_assign(c, ident, initializer),
        Expr::LetGet { ident } => compile_let_get(c, ident),
        Expr::LetSet { ident, expr } => compile_let_set(c, ident, expr),
        Expr::Fun { ident, decl } => compile_function(c, ident, decl),
        Expr::Call { callee, args } => compile_call(c, callee, args),
        Expr::While { condition, body } => compile_while(c, condition, body),
        Expr::IfElse {
            condition,
            then,
            else_,
        } => compile_if_else(c, condition, then, else_),
        Expr::Block { block } => compile_block(c, block),
        Expr::Print { expr } => compile_print(c, expr),
        Expr::Return { expr } => compile_return(c, expr),
        Expr::Literal(expr) => compile_literal(c, expr),
    }
}

fn compile_binary(compiler: &mut Compiler, left: Box<Expr>, op: BinaryOperator, right: Box<Expr>) {
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

    // Compile initializer.
    compile_expr(compiler, *init);

    compiler.define_variable(&ident);
}

fn compile_let_get(compiler: &mut Compiler, ident: Identifier) {
    if let Some(local) = compiler.resolve_local(&ident) {
        // Local variable
        compiler.emit(Opcode::GetLocal);
        compiler.emit_byte(local as u8);
    } else {
        // Global variable
        compiler.emit(Opcode::GetGlobal);
        let constant_id = compiler.add_constant(Value::String(ident));
        compiler.emit_byte(constant_id);
    }
}

fn compile_let_set(compiler: &mut Compiler, ident: Identifier, expr: Box<Expr>) {
    compile_expr(compiler, *expr);

    if let Some(local) = compiler.resolve_local(&ident) {
        // Local variable
        compiler.emit(Opcode::SetLocal);
        compiler.emit_byte(local as u8);
    } else {
        // Global variable
        compiler.emit(Opcode::SetGlobal);
        let constant_id = compiler.add_constant(Value::String(ident));
        compiler.emit_byte(constant_id);
    }
}

fn compile_function(compiler: &mut Compiler, ident: Identifier, decl: FunDecl) {
    compiler.set_instance(CompilerInstance::new(FunctionType::Function));

    compile_closure(compiler, &ident, decl);

    compiler.define_variable(&ident);
}

fn compile_closure(compiler: &mut Compiler, ident: &Identifier, decl: FunDecl) {
    compiler.begin_scope();

    let arity = decl.args().len();

    // Compile arguments.
    for arg in decl.args() {
        compiler.declare_variable(arg);
        compiler.define_variable(arg);
    }

    // Compile body.
    compile_expr(compiler, Expr::block(decl.body())); // TODO: Create new Block expr?

    // Create the function object.
    let mut fun = compiler.end_compiler();
    fun.set_name(ident.clone());
    fun.set_arity(arity as u8);

    compiler.emit(Opcode::Closure);

    let constant_id = compiler.add_constant(Value::Function(Gc::new(fun)));
    compiler.emit_byte(constant_id);
}

fn compile_call(compiler: &mut Compiler, callee: Box<Expr>, args: Vec<Expr>) {
    let arity = args.len();

    compile_expr(compiler, *callee);
    for arg in args {
        compile_expr(compiler, arg);
    }
    compiler.emit(Opcode::Call);
    compiler.emit_byte(arity as u8);
}

fn compile_while(compiler: &mut Compiler, condition: Box<Expr>, body: Box<Expr>) {
    let loop_start = compiler.current_chunk().code().len();
    compile_expr(compiler, *condition);

    let exit_jump = compiler.emit_jump(Opcode::JumpIfFalse);
    compiler.emit(Opcode::Pop);
    compile_expr(compiler, *body);

    compiler.emit_loop(loop_start);
    compiler.patch_jump(exit_jump);
    compiler.emit(Opcode::Pop);
}

fn compile_if_else(
    compiler: &mut Compiler,
    condition: Box<Expr>,
    then: BlockDecl,
    else_: Option<BlockDecl>,
) {
    compile_expr(compiler, *condition);

    // Jump to else clause if false.
    let then_jump = compiler.emit_jump(Opcode::JumpIfFalse);
    compiler.emit(Opcode::Pop);

    for expr in then {
        compile_expr(compiler, expr);
    }

    let else_jump = compiler.emit_jump(Opcode::Jump);

    compiler.patch_jump(then_jump);
    compiler.emit(Opcode::Pop);

    // Compile else clause if set.
    if let Some(exprs) = else_ {
        for expr in exprs {
            compile_expr(compiler, expr);
        }
    }

    compiler.patch_jump(else_jump);
}

fn compile_block(compiler: &mut Compiler, block: Box<BlockDecl>) {
    compiler.begin_scope();
    for expr in *block {
        compile_expr(compiler, expr);
    }
    compiler.end_scope();
}

fn compile_print(compiler: &mut Compiler, expr: Box<Expr>) {
    compile_expr(compiler, *expr);
    compiler.emit(Opcode::Print);
}

fn compile_return(compiler: &mut Compiler, expr: Option<Box<Expr>>) {
    if compiler.function_type() == &FunctionType::Script {
        compiler.add_error(CompilerError::InvalidReturn);
    }

    if let Some(expr) = expr {
        compile_expr(compiler, *expr);
        compiler.emit(Opcode::Return);
    } else {
        compiler.emit_return()
    }
}

fn compile_literal(compiler: &mut Compiler, literal: LiteralExpr) {
    match literal {
        LiteralExpr::Number(n) => compiler.emit_constant(Value::Number(n)),
        LiteralExpr::String(s) => compiler.emit_string(&s),
        LiteralExpr::True => compiler.emit_constant(Value::Bool(true)),
        LiteralExpr::False => compiler.emit_constant(Value::Bool(false)),
        LiteralExpr::Nil => todo!(), // TODO: Compile nil literals.
    }
}

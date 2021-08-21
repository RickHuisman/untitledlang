use crate::compiler::compiler::Compiler;
use crate::compiler::value::Value;
use crate::parser::ast::{BinaryOperator, BlockDecl, Expr, Identifier, LiteralExpr, UnaryOperator, FunDecl};
use crate::vm::opcode::Opcode;
use crate::compiler::object::{Closure, Function, FunctionType};
use crate::vm::obj::Gc;
use crate::compiler::instance::CompilerInstance;

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
        Expr::Block { block } => compile_block(c, block),
        Expr::Print { expr } => compile_print(c, expr),
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
        let constant_id = compiler.current_chunk().add_constant(Value::String(ident));
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
        let constant_id = compiler.current_chunk().add_constant(Value::String(ident));
        compiler.emit_byte(constant_id);
    }
}

fn compile_function(compiler: &mut Compiler, ident: Identifier, decl: FunDecl) {
    // compiler.declare_variable(&ident);
    // if compiler.is_scoped() {
    //     compiler.mark_local_initialized();
    // }
    //
    // compile_closure(compiler, &ident, decl);
    //
    // compiler.define_variable(&ident);

    let current_copy = compiler.current().clone();
    compiler.set_current(CompilerInstance::new(FunctionType::Function));
    *compiler.current_mut().enclosing_mut() = Box::new(Some(current_copy));

    // TODO: Set fun name.
    // *compiler.current_mut().function_mut().name_mut() = ident.clone();
    // *compiler.current_mut().function_mut().chunk_mut().name_mut() = Some(ident.clone());

    compiler.begin_scope();

    // Compile arguments.
    for arg in decl.args() {
        *compiler.current_mut().function_mut().arity_mut() += 1;
        compiler.declare_variable(arg);
    }

    // Compile body.
    compile_expr(compiler, Expr::block(decl.body()));

    // Create the function object.
    let fun = compiler.end_compiler();
    // TODO: Set fun name here.

    compiler.emit(Opcode::Closure);

    let constant_id = compiler
        .current_chunk()
        .add_constant(Value::Function(Gc::new(fun)));

    compiler.emit_byte(constant_id);

    compiler.define_variable(&ident); // TODO fun is always global?
}

fn compile_closure(compiler: &mut Compiler, ident: &Identifier, decl: FunDecl) {
    // let (chunk_index, upvalues) =
    //     compiler.with_scoped_context(context_type, |compiler| {
    //         for arg in args {
    //             declare_variable(compiler, &arg.value);
    //             define_variable(compiler, &arg.value);
    //         }
    //
    //         compile_ast(compiler, block);
    //
    //         {
    //             let expr: Option<Box<WithSpan<Expr>>> = None;
    //             compile_return(compiler, expr.as_ref());
    //         }
    //     });

    // let function = Function::new(
    //     name: identifier.value.into(),
    //     chunk_index,
    //     arity: decl..len(),
    // );
    //
    // // let closure = Closure::new(function);
    //
    // compiler.emit(Opcode::Closure);
    //
    // let constant_id = compiler
    //     .current_chunk()
    //     .add_constant(Value::Function(Gc::new(fun)));
    //
    // compiler.emit_byte(constant_id);

    // let constant = compiler.add_constant(Constant::Closure(closure));
    // compiler.add_instruction(Instruction::Closure(constant));
}

fn compile_call(compiler: &mut Compiler, callee: Box<Expr>, args: Vec<Expr>) {
    let arity = args.len();
    if arity > 8 {
        panic!() // TODO
    }

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

fn compile_literal(compiler: &mut Compiler, literal: LiteralExpr) {
    match literal {
        LiteralExpr::Number(n) => compiler.emit_constant(Value::Number(n)),
        LiteralExpr::String(s) => compiler.emit_string(&s),
        LiteralExpr::True => compiler.emit_constant(Value::Bool(true)),
        LiteralExpr::False => compiler.emit_constant(Value::Bool(false)),
        LiteralExpr::Nil => todo!(), // TODO: Compile nil literals.
    }
}

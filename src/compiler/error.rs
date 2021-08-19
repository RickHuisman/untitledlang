pub type Result<T> = std::result::Result<T, CompileError>;

#[derive(Debug)]
pub enum CompileError {
    LocalAlreadyDefined,
    LocalNotInitialized,
}

// panic!("Already a variable called {} in this scope.", ident); TODO

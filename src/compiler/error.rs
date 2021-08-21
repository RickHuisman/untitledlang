pub type CompileResult<T> = std::result::Result<T, CompilerError>;

#[derive(Debug)]
pub enum CompilerError {
    LocalAlreadyDefined,
    LocalNotInitialized,
    ReturnFromTopLevel,
}

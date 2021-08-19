pub type RunResult<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    ArgumentTypes,
    StackEmpty,
    FrameEmpty,
    InvalidCallee,
    IncorrectArity,
    BadStackIndex(usize, usize),
    UndefinedGlobal(String),
    ReturnFromTopLevel,
}

#[derive(Debug)]
pub enum RuntimeError {
    ArgumentTypes,
    StackEmpty,
    FrameEmpty,
    BadStackIndex(usize, usize),
    UndefinedGlobal(String),
    ReturnFromTopLevel,
}

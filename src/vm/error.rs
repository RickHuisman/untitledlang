#[derive(Debug)]
pub enum RuntimeError {
    StackEmpty,
    FrameEmpty,
    ReturnFromTopLevel,
}

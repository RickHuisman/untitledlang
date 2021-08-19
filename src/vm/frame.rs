use crate::compiler::object::Closure;
use crate::vm::obj::Gc;

#[derive(Clone)]
pub struct CallFrame {
    closure: Gc<Closure>,
    ip: usize,
    stack_start: usize,
}

impl CallFrame {
    pub fn new(closure: Gc<Closure>, stack_start: usize) -> Self {
        CallFrame {
            closure,
            ip: 0,
            stack_start,
        }
    }

    pub fn closure(&self) -> &Gc<Closure> {
        &self.closure
    }

    pub fn closure_mut(&mut self) -> &mut Gc<Closure> {
        &mut self.closure
    }

    pub fn ip(&self) -> &usize {
        &self.ip
    }

    pub fn ip_mut(&mut self) -> &mut usize {
        &mut self.ip
    }

    pub fn stack_start(&self) -> &usize {
        &self.stack_start
    }
}

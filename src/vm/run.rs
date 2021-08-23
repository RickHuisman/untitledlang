use crate::compiler::object::Closure;
use crate::compiler::value::Value;
use crate::vm::error::{RunResult, RuntimeError};
use crate::vm::opcode::Opcode;
use crate::vm::vm::VM;
use std::io::Write;

impl<W: Write> VM<W> {
    pub fn run(&mut self) -> RunResult<()> {
        while !self.is_at_end() {
            let instruction = Opcode::from(self.read_byte()?);
            match instruction {
                Opcode::Constant => self.constant()?,
                Opcode::Add => self.add()?,
                Opcode::Subtract => self.subtract()?,
                Opcode::Multiply => self.multiply()?,
                Opcode::Divide => self.divide()?,
                Opcode::Greater => self.greater()?,
                Opcode::Less => self.less()?,
                Opcode::Equal => self.equal()?,
                Opcode::Not => self.not()?,
                Opcode::Negate => self.negate()?,
                Opcode::GetLocal => self.get_local()?,
                Opcode::SetLocal => self.set_local()?,
                Opcode::DefineGlobal => self.define_global()?,
                Opcode::GetGlobal => self.get_global()?,
                Opcode::SetGlobal => self.set_global()?,
                Opcode::Jump => self.jump()?,
                Opcode::JumpIfFalse => self.jump_if_false()?,
                Opcode::Loop => self.loop_()?,
                Opcode::Return => self.ret()?,
                Opcode::Closure => self.closure()?,
                Opcode::Call => self.call_instr()?,
                Opcode::Print => self.print()?,
                Opcode::Pop => {
                    self.pop()?;
                }
            }
        }

        Ok(())
    }

    fn constant(&mut self) -> RunResult<()> {
        let constant = self.read_constant()?.clone();
        self.push(constant);
        Ok(())
    }

    fn add(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a + b);
        Ok(())
    }

    fn subtract(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a - b);
        Ok(())
    }

    fn multiply(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a * b);
        Ok(())
    }

    fn divide(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a / b);
        Ok(())
    }

    fn equal(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push((a == b).into());
        Ok(())
    }

    fn greater(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push((a > b).into());
        Ok(())
    }

    fn less(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push((a < b).into());
        Ok(())
    }

    fn not(&mut self) -> RunResult<()> {
        let a = self.pop()?;
        self.push(bool::into(!bool::from(&a)));
        Ok(())
    }

    fn negate(&mut self) -> RunResult<()> {
        let a = self.pop()?;
        self.push(-a);
        Ok(())
    }

    fn get_local(&mut self) -> RunResult<()> {
        let start = *self.frame()?.stack_start();
        let slot = self.read_byte()? as usize;
        let index = start + slot;

        if let Some(value) = self.stack().get(index).cloned() {
            self.stack_mut().push(value);
            return Ok(());
        }

        Err(RuntimeError::BadStackIndex(index, self.stack().len()))
    }

    fn set_local(&mut self) -> RunResult<()> {
        let value = self.peek()?.clone();
        let start = *self.frame()?.stack_start();
        let slot = self.read_byte()? as usize;
        self.stack_mut()[start + slot] = value;
        Ok(())
    }

    fn define_global(&mut self) -> RunResult<()> {
        if let Ok(value) = self.pop() {
            let var_name = self.read_string()?;
            self.globals_mut().insert(var_name, value);
            return Ok(());
        }

        Err(RuntimeError::BadStackIndex(10, self.stack().len())) // TODO 10
    }

    fn get_global(&mut self) -> RunResult<()> {
        let name = self.read_string()?;

        if let Some(value) = self.globals().get(&name).cloned() {
            self.push(value);
            return Ok(());
        }

        Err(RuntimeError::UndefinedGlobal(name))
    }

    fn set_global(&mut self) -> RunResult<()> {
        let name = self.read_string()?;

        if self.globals().contains_key(&name) {
            let value = self.peek()?.clone();
            self.globals_mut().insert(name, value);
            return Ok(());
        }

        Err(RuntimeError::UndefinedGlobal(name))
    }

    fn jump(&mut self) -> RunResult<()> {
        let offset = self.read_short()?;
        *self.frame_mut()?.ip_mut() += offset as usize;
        Ok(())
    }

    fn jump_if_false(&mut self) -> RunResult<()> {
        let offset = self.read_short()?;

        if !bool::from(self.peek()?) {
            *self.frame_mut()?.ip_mut() += offset as usize;
        }
        Ok(())
    }

    fn loop_(&mut self) -> RunResult<()> {
        let offset = self.read_short()?;
        *self.frame_mut()?.ip_mut() -= offset as usize;
        Ok(())
    }

    fn ret(&mut self) -> RunResult<()> {
        if let Some(frame) = self.frames_mut().pop() {
            let result = self.pop()?;
            self.stack_mut().truncate(*frame.stack_start());
            self.push(result);
            return Ok(());
        }

        Err(RuntimeError::ReturnFromTopLevel)
    }

    fn closure(&mut self) -> RunResult<()> {
        let closure = Closure::new(self.read_function()?);
        let clos = self.alloc(closure);
        self.push(Value::Closure(clos));
        Ok(())
    }

    fn call_instr(&mut self) -> RunResult<()> {
        let arity = self.read_byte()?;
        self.call_value(arity)?;
        Ok(())
    }

    fn print(&mut self) -> RunResult<()> {
        let popped = self.pop()?;
        writeln!(self.stdout_mut(), "{}", popped);
        Ok(())
    }
}

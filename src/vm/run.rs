use crate::vm::vm::VM;
use crate::vm::vm::Result;
use crate::vm::opcode::Opcode;
use crate::vm::error::RuntimeError;
use std::io::Write;

impl<W: Write> VM<W> {
    pub fn run(&mut self) -> Result<()> {
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
                Opcode::GetGlobal => self.get_global()?,
                Opcode::SetGlobal => self.set_global()?,
                Opcode::DefineGlobal => self.define_global()?,
                Opcode::Return => self.ret()?,
                Opcode::Print => self.print()?,
                Opcode::Pop => { self.pop()?; }
            }
        }

        Ok(())
    }

    fn constant(&mut self) -> Result<()> {
        let constant = self.read_constant()?.clone();
        self.push(constant);
        Ok(())
    }

    fn add(&mut self) -> Result<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a + b);
        Ok(())
    }

    fn subtract(&mut self) -> Result<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a - b);
        Ok(())
    }

    fn multiply(&mut self) -> Result<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a * b);
        Ok(())
    }

    fn divide(&mut self) -> Result<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a / b);
        Ok(())
    }

    fn equal(&mut self) -> Result<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push((a == b).into());
        Ok(())
    }

    fn greater(&mut self) -> Result<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push((a > b).into());
        Ok(())
    }

    fn less(&mut self) -> Result<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push((a < b).into());
        Ok(())
    }

    fn not(&mut self) -> Result<()> {
        let a = self.pop()?;
        self.push(bool::into(!bool::from(&a)));
        Ok(())
    }

    fn negate(&mut self) -> Result<()> {
        let a = self.pop()?;
        self.push(-a);
        Ok(())
    }

    fn get_local(&mut self) -> Result<()> {
        let start = *self.frame()?.stack_start();
        let slot = self.read_byte()? as usize;
        let index = start + slot;

        if let Some(value) = self.stack().get(index).cloned() {
            self.stack_mut().push(value);
            Ok(())
        } else {
            Err(RuntimeError::BadStackIndex(index, self.stack().len()))
        }
    }

    fn set_local(&mut self) -> Result<()> {
        let value = self.peek()?.clone();
        let start = *self.frame()?.stack_start();
        let slot = self.read_byte()? as usize;
        self.stack_mut()[start + slot] = value;
        Ok(())
    }

    fn define_global(&mut self) -> Result<()> {
        if let Ok(value) = self.pop() {
            let var_name = self.read_string()?;
            self.globals_mut().insert(var_name, value);
            Ok(())
        } else {
            Err(RuntimeError::BadStackIndex(10, self.stack().len())) // TODO 10
        }
    }

    fn get_global(&mut self) -> Result<()> {
        let name = self.read_string()?;

        if let Some(value) = self.globals_mut().get(&name).cloned() {
            self.push(value);
            Ok(())
        } else {
            Err(RuntimeError::UndefinedGlobal(name))
        }
    }

    fn set_global(&mut self) -> Result<()> {
        // let name = self.read_constant()?.clone().as_string()?; // TODO: Works?
        let name = self.read_string()?;

        if self.globals().contains_key(&name) {
            let value = self.peek()?.clone();
            self.globals_mut().insert(name, value);
            return Ok(());
        }

        Err(RuntimeError::UndefinedGlobal(name))
    }

    fn ret(&mut self) -> Result<()> {
        if let Some(frame) = self.frames_mut().pop() {
            let result = self.pop()?;
            self.stack_mut().truncate(*frame.stack_start());
            self.push(result);
            Ok(())
        } else {
            Err(RuntimeError::ReturnFromTopLevel)
        }
    }

    fn print(&mut self) -> Result<()> {
        let popped = self.pop()?;
        writeln!(self.stdout_mut(), "{}", popped);
        Ok(())
    }
}

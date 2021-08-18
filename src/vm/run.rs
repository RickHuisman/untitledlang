use crate::vm::vm::VM;
use crate::vm::vm::Result;
use crate::vm::opcode::Opcode;
use crate::vm::error::RuntimeError;

impl VM {
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
        println!("{:?}", popped);
        Ok(())
    }
}

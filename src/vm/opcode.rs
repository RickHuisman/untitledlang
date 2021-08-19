use crate::parser::ast::UnaryOperator;

pub type ConstantIndex = usize;
pub type StackIndex = usize;

#[repr(u8)]
pub enum Opcode {
    Return,
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Greater,
    Less,
    Not,
    Negate,
    GetLocal,
    SetLocal,
    DefineGlobal,
    GetGlobal,
    SetGlobal,
    Print,
    Pop,
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Opcode::Return,
            0x01 => Opcode::Constant,
            0x02 => Opcode::Add,
            0x03 => Opcode::Subtract,
            0x04 => Opcode::Multiply,
            0x05 => Opcode::Divide,
            0x06 => Opcode::Equal,
            0x07 => Opcode::Greater,
            0x08 => Opcode::Less,
            0x09 => Opcode::Not,
            0x0a => Opcode::Negate,
            0x0b => Opcode::GetLocal,
            0x0c => Opcode::SetLocal,
            0x0d => Opcode::DefineGlobal,
            0x0e => Opcode::GetGlobal,
            0x0f => Opcode::SetGlobal,
            0x10 => Opcode::Print,
            0x11 => Opcode::Pop,
            _ => panic!("No opcode for byte: {}", byte), // TODO: Option?
        }
    }
}

impl From<UnaryOperator> for Opcode {
    fn from(op: UnaryOperator) -> Self {
        match op {
            UnaryOperator::Negate => Opcode::Negate,
            UnaryOperator::Not => Opcode::Not,
        }
    }
}

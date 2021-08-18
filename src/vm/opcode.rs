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
            0x0a => Opcode::Print,
            0x0b => Opcode::Pop,
            _ => panic!("No opcode for byte: {}", byte), // TODO: Option?
        }
    }
}

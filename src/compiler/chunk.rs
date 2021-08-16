use crate::compiler::value::Value;
use crate::vm::opcode::Opcode;

#[derive(Clone)]
pub struct Chunk {
    name: Option<String>,
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            name: None,
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn write(&mut self, opcode: Opcode, line: usize) {
        self.lines.push(line);
        self.write_byte(opcode as u8);
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        self.constants.len() as u8 - 1
    }

    pub fn name_mut(&mut self) -> &mut Option<String> {
        &mut self.name
    }

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn code_mut(&mut self) -> &mut Vec<u8> {
        &mut self.code
    }

    pub fn read_constant(&self, index: usize) -> &Value {
        &self.constants[index]
    }

    pub fn constants(&self) -> &Vec<Value> {
        &self.constants
    }
}

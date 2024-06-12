pub struct Instruction {
    pub opcode: OpCode,
}

impl Instruction {
    pub fn new(opcode: OpCode) -> Instruction {
        Instruction { opcode }
    }
}

pub enum DataType {
    Int64,
}

pub enum OpCode {
    Zero,
    LoadConst { dataType: DataType },
}

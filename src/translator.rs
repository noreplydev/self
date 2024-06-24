use crate::{instructions::Instruction, types::DataType};

pub struct Translator {
    bytecode: Vec<u8>,
    pc: usize,
}

impl Translator {
    pub fn new(bytecode: Vec<u8>) -> Translator {
        Translator { bytecode, pc: 0 }
    }

    pub fn translate(&mut self) -> Vec<Instruction> {
        let mut instructions = vec![];

        while self.pc < self.bytecode.len() {
            match self.bytecode[self.pc] {
                // ZERO
                0 => instructions.push(Instruction::Zero),
                // LOAD_CONST
                0x01 => {
                    // check for register index, data type and value
                    if self.pc + 1 >= self.bytecode.len() {
                        panic!("Invalid LoadConst instruction at position {}", self.pc);
                    }

                    let data_type = match self.bytecode[self.pc + 1] {
                        0x00 => DataType::Nothing,
                        0x01 => DataType::U32,
                        0x02 => DataType::U64,
                        0x03 => DataType::I32,
                        0x04 => DataType::I64,
                        _ => panic!("Unknown data type"),
                    };

                    let value_length = match data_type {
                        DataType::I32 => 4,
                        DataType::I64 => 8,
                        DataType::U32 => 4,
                        DataType::U64 => 8,
                        DataType::Nothing => 0,
                    };
                    if (self.pc + 1 + value_length) >= self.bytecode.len() {
                        panic!("Invalid value size at position {}", self.pc + 2);
                    };

                    let value_bytes =
                        self.bytecode[self.pc + 2..self.pc + 2 + value_length].to_vec();

                    instructions.push(Instruction::LoadConst {
                        data_type,
                        value: value_bytes,
                    });

                    // increment program counter
                    // by the seeked bytes
                    self.pc += 1 + value_length;
                }
                // PRINT
                0x02 => {
                    // get u32 value. 4 bytes based on the type plus the current
                    let value_length = 4;
                    if self.pc + value_length >= self.bytecode.len() {
                        panic!("Invalid print instruction at position {}", self.pc);
                    }

                    let value_bytes = &self.bytecode[self.pc + 1..self.pc + 5];
                    let number_of_args = u32::from_le_bytes(
                        value_bytes.try_into().expect("Provided value is incorrect"),
                    );
                    instructions.push(Instruction::Print { number_of_args });
                    self.pc += 4;
                }
                // ADD
                0x03 => instructions.push(Instruction::Add),
                _ => {}
            };

            self.pc += 1;
        }

        instructions
    }
}

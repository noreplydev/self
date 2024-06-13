mod development;
mod instructions;
mod types;

use development::*;
use instructions::*;
use types::*;

struct Vm {
    stack: Vec<Value>,
    instructions: Vec<Instruction>,
    pc: usize,
}

impl Vm {
    fn new(bytecode: Vec<u8>) -> Vm {
        let mut instructions = vec![];
        let mut pc = 0;

        while pc < bytecode.len() {
            match bytecode[pc] {
                // zero
                0 => instructions.push(Instruction::Zero),
                // loadConst
                0x01 => {
                    // check for register index, data type and value
                    if pc >= bytecode.len() {
                        panic!("Invalid LoadConst instruction at position {}", pc);
                    }

                    // todo: check if register position not exceds the limit
                    let reg = bytecode[pc + 1] as usize;
                    let data_type = match bytecode[pc + 2] {
                        0x01 => DataType::Int64,
                        _ => panic!("Unknown data type"),
                    };

                    let value_length = match data_type {
                        DataType::Int64 => 8,
                    };
                    if (pc + 2 + value_length) >= bytecode.len() {
                        panic!("Invalid value size at position {}", pc + 3);
                    };

                    let value_bytes = bytecode[pc + 3..pc + 3 + value_length].to_vec();

                    instructions.push(Instruction::LoadConst {
                        reg,
                        data_type,
                        value: value_bytes,
                    });
                    // increment program counter
                    // by the seeked bytes
                    pc += 2 + value_length;
                }
                _ => {}
            };

            pc += 1;
        }

        Vm {
            stack: vec![],
            instructions,
            pc: 0,
        }
    }

    fn run(&mut self) {
        while self.pc < self.instructions.len() {
            let instruction = &self.instructions[self.pc];
            match &instruction {
                Instruction::Zero => {
                    println!("Zero");
                }
                Instruction::LoadConst {
                    reg,
                    data_type,
                    value,
                } => {
                    println!("LoadConst ({reg}) <- {:?}", data_type);
                }
            }

            self.pc += 1; // increment program counter
        }
    }
}

fn main() {
    let mut instructions: Vec<u8> = vec![0x01, 0x00, 0x01];
    instructions.extend_from_slice(&u64_to_bytes(14));

    let mut vm = Vm::new(instructions);
    vm.run();
}

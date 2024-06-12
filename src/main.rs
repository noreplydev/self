mod development;

use development::*;

struct Instruction {
    opcode: OpCode,
}

impl Instruction {
    fn new(opcode: OpCode) -> Instruction {
        Instruction { opcode }
    }
}

enum OpCode {
    Zero,
    LoadConst,
}

struct Vm {
    instructions: Vec<Instruction>,
}

impl Vm {
    fn new(instructions: Vec<u8>) -> Vm {
        let mut semantic_instructions = vec![];
        for instruction in instructions {
            match instruction {
                0 => semantic_instructions.push(Instruction::new(OpCode::Zero)),
                0x01 => semantic_instructions.push(Instruction::new(OpCode::LoadConst)),
                _ => {}
            };
        }

        Vm {
            instructions: semantic_instructions,
        }
    }

    fn run(&self) {
        for instruction in &self.instructions {
            match instruction.opcode {
                OpCode::Zero => {
                    println!("Zero");
                }
                OpCode::LoadConst => {
                    println!("LoadConst");
                }
            }
        }
    }
}

fn main() {
    let mut instructions: Vec<u8> = vec![0x01];
    instructions.extend_from_slice(&u64_to_bytes(14));

    let vm = Vm::new(instructions);
    vm.run();
}

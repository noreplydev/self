mod development;
mod instructions;

use development::*;
use instructions::*;

struct Vm {
    instructions: Vec<Instruction>,
}

impl Vm {
    fn new(instructions: Vec<u8>) -> Vm {
        let mut semantic_instructions = vec![];
        for instruction in instructions {
            match &instruction {
                // zero
                0 => semantic_instructions.push(Instruction::new(OpCode::Zero)),
                // loadConst
                0x01 => {}
                _ => {}
            };
        }

        Vm {
            instructions: semantic_instructions,
        }
    }

    fn run(&self) {
        for instruction in &self.instructions {
            match &instruction.opcode {
                OpCode::Zero => {
                    println!("Zero");
                }
                OpCode::LoadConst { dataType } => {
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

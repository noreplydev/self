mod development;
mod instructions;

use development::*;
use instructions::*;

struct Vm {
    instructions: Vec<Instruction>,
    pc: usize,
}

impl Vm {
    fn new(instructions: Vec<u8>) -> Vm {
        let mut semantic_instructions = vec![];
        let mut pc = 0;

        while pc < instructions.len() {
            match instructions[pc] {
                // zero
                0 => semantic_instructions.push(Instruction::Zero),
                // loadConst
                0x01 => {}
                _ => {}
            };

            pc += 1;
        }

        Vm {
            instructions: semantic_instructions,
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
                Instruction::LoadConst { dataType } => {
                    println!("LoadConst");
                }
            }

            self.pc += 1; // increment program counter
        }
    }
}

fn main() {
    let mut instructions: Vec<u8> = vec![0x01];
    instructions.extend_from_slice(&u64_to_bytes(14));

    let mut vm = Vm::new(instructions);
    vm.run();
}

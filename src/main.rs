struct Instruction {
    opcode: OpCode,
}

impl Instruction {
    fn new(opcode: OpCode) -> Instruction {
        Instruction { opcode }
    }
}

enum OpCode {
    LoadConst,
}

struct Vm {
    instructions: Vec<Instruction>,
}

impl Vm {
    fn new(instructions: Vec<Instruction>) -> Vm {
        Vm { instructions }
    }
    fn run(&self) {
        for instruction in &self.instructions {
            match instruction.opcode {
                OpCode::LoadConst => {
                    println!("LoadConst");
                }
            }
        }
    }
}

fn main() {
    let instructions = vec![Instruction::new(OpCode::LoadConst)];
    let vm = Vm::new(instructions);

    vm.run()
}

use super::instructions::*;
use super::types::*;

pub struct Vm {
    operand_stack: Vec<Value>,
    instructions: Vec<Instruction>,
    pc: usize,
}

impl Vm {
    pub fn new(bytecode: Vec<u8>) -> Vm {
        let mut instructions = vec![];
        let mut pc = 0;

        while pc < bytecode.len() {
            match bytecode[pc] {
                // ZERO
                0 => instructions.push(Instruction::Zero),
                // LOAD_CONST
                0x01 => {
                    // check for register index, data type and value
                    if pc + 1 >= bytecode.len() {
                        panic!("Invalid LoadConst instruction at position {}", pc);
                    }

                    // todo: check if register position not exceds the limit
                    let data_type = match bytecode[pc + 1] {
                        0x01 => DataType::Int64,
                        _ => panic!("Unknown data type"),
                    };

                    let value_length = match data_type {
                        DataType::Int64 => 8,
                    };
                    if (pc + 1 + value_length) >= bytecode.len() {
                        panic!("Invalid value size at position {}", pc + 2);
                    };

                    let value_bytes = bytecode[pc + 2..pc + 2 + value_length].to_vec();

                    instructions.push(Instruction::LoadConst {
                        data_type,
                        value: value_bytes,
                    });

                    // increment program counter
                    // by the seeked bytes
                    pc += 1 + value_length;
                }
                // ADD
                0x02 => instructions.push(Instruction::Add),
                _ => {}
            };

            pc += 1;
        }

        Vm {
            operand_stack: vec![],
            instructions,
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.instructions.len() {
            let instruction = &self.instructions[self.pc];
            match &instruction {
                Instruction::Zero => {
                    println!("Zero");
                }
                Instruction::LoadConst { data_type, value } => {
                    let printable_value;
                    self.operand_stack.push(match data_type {
                        DataType::Int64 => {
                            let value = i64::from_le_bytes(
                                value
                                    .as_slice()
                                    .try_into()
                                    .expect("Provided value is incorrect"),
                            );
                            printable_value = value;
                            Value::I64(I64::new(value))
                        }
                    });
                    println!("LOAD_CONST <- {:?}({printable_value})", data_type);
                }
                Instruction::Add => {
                    let right_operand = self.operand_stack.pop();
                    let left_operand = self.operand_stack.pop();

                    if left_operand.is_none() || right_operand.is_none() {
                        panic!("Operands stack underflow");
                    };

                    let operands = (left_operand.unwrap(), right_operand.unwrap());
                    let operands_types = (operands.0.get_type(), operands.1.get_type());

                    if operands_types.0 != operands_types.1 {
                        panic!("Operands type mismatch");
                    }

                    match operands {
                        (Value::I64(l), Value::I64(r)) => {
                            self.operand_stack
                                .push(Value::I64(I64::new(l.value + r.value)));
                            println!("ADD -> {:?}", l.value + r.value);
                        }
                    }
                }
            }

            self.pc += 1; // increment program counter
        }
    }
}

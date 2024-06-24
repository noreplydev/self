use super::instructions::*;
use super::symbol_table::*;
use super::types::*;

use self::i32::I32;
use self::i64::I64;
use self::u32::U32;
use self::u64::U64;
pub struct Vm {
    operand_stack: Vec<Value>,
    symbol_table: SymbolTable,
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

                    let data_type = match bytecode[pc + 1] {
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
                // PRINT
                0x02 => {
                    // get u32 value. 4 bytes based on the type plus the current
                    let value_length = 4;
                    if pc + value_length >= bytecode.len() {
                        panic!("Invalid print instruction at position {}", pc);
                    }

                    let value_bytes = &bytecode[pc + 1..pc + 5];
                    let number_of_args = u32::from_le_bytes(
                        value_bytes.try_into().expect("Provided value is incorrect"),
                    );
                    instructions.push(Instruction::Print { number_of_args });
                    pc += 4;
                }
                // ADD
                0x03 => instructions.push(Instruction::Add),
                _ => {}
            };

            pc += 1;
        }

        Vm {
            operand_stack: vec![],
            symbol_table: SymbolTable::new(),
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
                        DataType::I32 => {
                            let value = i32::from_le_bytes(
                                value
                                    .as_slice()
                                    .try_into()
                                    .expect("Provided value is incorrect"),
                            );
                            printable_value = value.to_string();
                            Value::I32(I32::new(value))
                        }
                        DataType::I64 => {
                            let value = i64::from_le_bytes(
                                value
                                    .as_slice()
                                    .try_into()
                                    .expect("Provided value is incorrect"),
                            );
                            printable_value = value.to_string();
                            Value::I64(I64::new(value))
                        }
                        DataType::U32 => {
                            let value = u32::from_le_bytes(
                                value
                                    .as_slice()
                                    .try_into()
                                    .expect("Provided value is incorrect"),
                            );
                            printable_value = value.to_string();
                            Value::U32(U32::new(value))
                        }
                        DataType::U64 => {
                            let value = u64::from_le_bytes(
                                value
                                    .as_slice()
                                    .try_into()
                                    .expect("Provided value is incorrect"),
                            );
                            printable_value = value.to_string();
                            Value::U64(U64::new(value))
                        }
                        DataType::Nothing => {
                            printable_value = "nothing".to_string();
                            Value::Nothing
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
                        panic!("No explicit coercion. Operands type mismatch.");
                    }

                    match operands {
                        (Value::I32(l), Value::I32(r)) => {
                            self.operand_stack
                                .push(Value::I32(I32::new(l.value + r.value)));
                            println!("ADD -> {:?}", l.value + r.value);
                        }
                        (Value::I64(l), Value::I64(r)) => {
                            self.operand_stack
                                .push(Value::I64(I64::new(l.value + r.value)));
                            println!("ADD -> {:?}", l.value + r.value);
                        }
                        (Value::U32(l), Value::U32(r)) => {
                            self.operand_stack
                                .push(Value::U32(U32::new(l.value + r.value)));
                            println!("ADD -> {:?}", l.value + r.value);
                        }
                        (Value::Nothing, Value::Nothing) => {
                            self.operand_stack.push(Value::Nothing);
                            println!("ADD -> nothing");
                        }
                        _ => unreachable!(),
                    }
                }
                Instruction::Print { number_of_args } => {
                    let mut counter = 0;
                    while &counter < number_of_args {
                        if let Some(v) = self.operand_stack.pop() {
                            match v {
                                Value::I32(x) => println!("PRINT -> {}", x.value),
                                Value::I64(x) => println!("PRINT -> {}", x.value),
                                Value::U32(x) => println!("PRINT -> {}", x.value),
                                Value::U64(x) => println!("PRINT -> {}", x.value),
                                Value::Nothing => println!("PRINT -> nothing"),
                                // Handle other types as necessary
                            }
                        } else {
                            panic!("Cannot get arg to print")
                        }
                        counter += 1;
                    }
                }
            }

            self.pc += 1; // increment program counter
        }
    }
}

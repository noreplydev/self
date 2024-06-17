use crate::types::DataType;

#[derive(Debug)]
pub enum Instruction {
    Zero,
    LoadConst { data_type: DataType, value: Vec<u8> },
    Add,
    Print { number_of_args: u32 },
}

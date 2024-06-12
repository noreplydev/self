#[derive(Debug)]
pub enum DataType {
    Int64,
}

#[derive(Debug)]
pub enum Instruction {
    Zero,
    LoadConst {
        reg: usize,
        data_type: DataType,
        value: Vec<u8>,
    },
}

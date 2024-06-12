pub enum DataType {
    Int64,
}

pub enum Instruction {
    Zero,
    LoadConst { dataType: DataType },
}

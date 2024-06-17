pub mod i64;
pub mod u32;

use i64::I64;
use u32::U32;

#[derive(Debug)]
pub enum Value {
    I64(I64),
    U32(U32),
    Nothing,
}

impl Value {
    pub fn get_type(&self) -> DataType {
        match self {
            Value::I64(_) => DataType::Int64,
            Value::U32(_) => DataType::U32,
            Value::Nothing => DataType::Nothing,
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    Int64,
    U32,
    Nothing,
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataType::Int64, DataType::Int64) => true,
            (DataType::U32, DataType::U32) => true,
            (DataType::Nothing, DataType::Nothing) => true,
            _ => false,
        }
    }
}

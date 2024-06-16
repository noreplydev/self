mod i64;

pub use i64::I64;

#[derive(Debug)]
pub enum Value {
    I64(I64),
}

impl Value {
    pub fn get_type(&self) -> DataType {
        match self {
            Value::I64(_) => DataType::Int64,
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    Int64,
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataType::Int64, DataType::Int64) => true,
            _ => false,
        }
    }
}

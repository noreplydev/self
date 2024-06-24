use crate::types::Value;
use std::collections::HashMap;

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Value>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            scopes: vec![HashMap::new()],
        }
    }
}

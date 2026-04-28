use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Int(i64),
    Text(String),
}
impl Value {
    pub fn to_string_cmp(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Text(s) => s.clone(),
        }
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Text(s) => write!(f, "{}", s),
        }
    }
}
pub type Row = Vec<Value>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColumnType {
    Int,
    Text,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub columns: Vec<String>,
    pub types: Vec<ColumnType>,
}
impl Schema {
    pub fn new(columns: Vec<String>) -> Self {
        let types = vec![ColumnType::Text; columns.len()];
        Self { columns, types }
    }
}

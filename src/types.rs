use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    Int(i64),
}
impl Value {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Value::Text(s) => s.clone(),
            Value::Int(i) => i.to_string(),
        }
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        if let Ok(i) = s.parse::<i64>() {
            Value::Int(i)
        } else {
            Value::Text(s.to_string())
        }
    }
}
impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::from(s.as_str())
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Text(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
        }
    }
}
pub type Row = Vec<Value>;
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ColumnType {
    Text,
    Int,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableSchema {
    pub columns: Vec<String>,
    pub types: Vec<ColumnType>,
}
impl TableSchema {
    pub fn new(columns: Vec<String>) -> Self {
        let types = vec![ColumnType::Text; columns.len()];
        Self { columns, types }
    }
}

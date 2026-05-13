use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Int(i64),
    Text(String),
    Bool(bool),
    Float(f64),
    Null,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Csv,
    Psv,
    Json,
    Html,
    Markdown,
    SqliteDump,
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Text(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Null => write!(f, "null"),
        }
    }
}
impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Text(s.to_string())
    }
}
impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}
impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

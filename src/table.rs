use crate::error::{Error, Result};
use crate::types::Value;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
    Int,
    Text,
    Bool,
    Float,
}
impl ColumnType {
    pub fn validate(&self, value: &Value) -> bool {
        match (self, value) {
            (ColumnType::Int, Value::Int(_)) => true,
            (ColumnType::Text, Value::Text(_)) => true,
            (ColumnType::Bool, Value::Bool(_)) => true,
            (ColumnType::Float, Value::Float(_)) => true,
            _ => false,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            ColumnType::Int => "INT",
            ColumnType::Text => "TEXT",
            ColumnType::Bool => "BOOL",
            ColumnType::Float => "FLOAT",
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDef {
    pub name: String,
    pub col_type: ColumnType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnDef>,
}
impl TableSchema {
    pub fn new(name: String, columns: Vec<ColumnDef>) -> Self {
        Self { name, columns }
    }
    pub fn validate_row(&self, row: &[Value]) -> Result<()> {
        if row.len() != self.columns.len() {
            return Err(Error::InvalidInput(format!(
                "Expected {} columns, got {}",
                self.columns.len(),
                row.len()
            )));
        }
        for (_idx, (col_def, val)) in self.columns.iter().zip(row.iter()).enumerate() {
            if !col_def.col_type.validate(val) {
                return Err(Error::TypeMismatch {
                    column: col_def.name.clone(),
                    expected: col_def.col_type.name().to_string(),
                    actual: match val {
                        Value::Int(_) => "INT",
                        Value::Text(_) => "TEXT",
                        Value::Bool(_) => "BOOL",
                        Value::Float(_) => "FLOAT",
                        Value::Null => "NULL",
                    }
                    .to_string(),
                });
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub schema: TableSchema,
    pub rows: Vec<Vec<Value>>,
}
impl TableData {
    pub fn new(schema: TableSchema) -> Self {
        Self {
            schema,
            rows: vec![],
        }
    }
}

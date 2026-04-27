use super::load_scema::load_schema;
use crate::config;
use crate::error::{NeuxDbError, Result};
use crate::types::{ColumnType, Row, Value};
use csv::ReaderBuilder;
use std::fs::{self, File};
pub fn read_table(name: &str) -> Result<(Vec<String>, Vec<Row>)> {
    let path = config::table_path(name)?;
    if !path.exists() {
        return Err(NeuxDbError::TableNotFound(name.to_string()));
    }
    let file = File::open(&path)?;
    file.lock_shared()
        .map_err(|e| NeuxDbError::Lock(format!("Failed to acquire shared lock: {}", e)))?;
    let data = fs::read_to_string(&path)?;
    file.unlock()
        .map_err(|e| NeuxDbError::Lock(format!("Failed to unlock: {}", e)))?;
    let schema = load_schema(name)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(config::delimiter_byte())
        .from_reader(data.as_bytes());
    let headers = rdr.headers()?.iter().map(|s| s.to_string()).collect();
    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut row = Vec::with_capacity(record.len());
        for (i, field) in record.iter().enumerate() {
            let expected_type = schema.types.get(i).cloned().unwrap_or(ColumnType::Text);
            let val = match expected_type {
                ColumnType::Int => {
                    if let Ok(num) = field.parse::<i64>() {
                        Value::Int(num)
                    } else {
                        Value::Text(field.to_string())
                    }
                }
                ColumnType::Text => Value::Text(field.to_string()),
            };
            row.push(val);
        }
        rows.push(row);
    }
    Ok((headers, rows))
}

use crate::config;
use crate::error::{NeuxDbError, Result};
use crate::types::{ColumnType, Row, TableSchema, Value};
use csv::{ReaderBuilder, WriterBuilder};
use fs2::FileExt;
use std::fs::{self, File};
fn load_schema(name: &str) -> Result<TableSchema> {
    let schema_path = config::schema_path(name)?;
    if !schema_path.exists() {
        let path = config::table_path(name)?;
        if !path.exists() {
            return Err(NeuxDbError::TableNotFound(name.to_string()));
        }
        let data = fs::read_to_string(&path)?;
        let mut rdr = ReaderBuilder::new()
            .delimiter(config::delimiter_byte())
            .from_reader(data.as_bytes());
        let headers = rdr.headers()?.iter().map(|s| s.to_string()).collect();
        let schema = TableSchema::new(headers);
        save_schema(name, &schema)?;
        Ok(schema)
    } else {
        let content = fs::read_to_string(schema_path)?;
        let schema: TableSchema =
            serde_json::from_str(&content).map_err(|e| NeuxDbError::Schema(e.to_string()))?;
        Ok(schema)
    }
}
fn save_schema(name: &str, schema: &TableSchema) -> Result<()> {
    let schema_path = config::schema_path(name)?;
    let content =
        serde_json::to_string_pretty(schema).map_err(|e| NeuxDbError::Schema(e.to_string()))?;
    fs::write(schema_path, content)?;
    Ok(())
}
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
pub fn write_table(name: &str, headers: &[String], rows: &[Row]) -> Result<()> {
    let path = config::table_path(name)?;
    let file = File::create(&path)?;
    file.lock_exclusive()
        .map_err(|e| NeuxDbError::Lock(format!("Failed to acquire exclusive lock: {}", e)))?;
    let mut buf = Vec::new();
    {
        let mut wtr = WriterBuilder::new()
            .delimiter(config::delimiter_byte())
            .from_writer(&mut buf);
        wtr.write_record(headers)?;
        for row in rows {
            let str_row: Vec<String> = row.iter().map(|v| v.to_string()).collect();
            wtr.write_record(&str_row)?;
        }
        wtr.flush()?;
    }
    let plain =
        String::from_utf8(buf).map_err(|e| NeuxDbError::Parse(format!("Invalid UTF-8: {}", e)))?;
    fs::write(&path, plain)?;
    file.unlock()
        .map_err(|e| NeuxDbError::Lock(format!("Failed to unlock: {}", e)))?;
    Ok(())
}
pub fn create_table_schema(name: &str, columns: &[String]) -> Result<()> {
    let path = config::table_path(name)?;
    if path.exists() {
        return Err(NeuxDbError::TableAlreadyExists(name.to_string()));
    }
    let schema = TableSchema::new(columns.to_vec());
    save_schema(name, &schema)?;
    let file = File::create(&path)?;
    file.lock_exclusive()
        .map_err(|e| NeuxDbError::Lock(format!("Failed to lock: {}", e)))?;
    let mut wtr = WriterBuilder::new()
        .delimiter(config::delimiter_byte())
        .from_writer(file);
    wtr.write_record(columns)?;
    wtr.flush()?;
    Ok(())
}

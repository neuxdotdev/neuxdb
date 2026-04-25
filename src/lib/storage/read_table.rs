use crate::config::{table_path, DELIMITER};
use crate::error::{NeuxError, Result};
use crate::types::{Row, Value};
use csv::ReaderBuilder;
use std::fs;
pub fn read_table(name: &str) -> Result<(Vec<String>, Vec<Row>)> {
    let path = table_path(name);
    if !path.exists() {
        return Err(NeuxError::TableNotFound(name.to_string()));
    }
    let data = fs::read_to_string(&path)?;
    let plain = data;
    let mut rdr = ReaderBuilder::new()
        .delimiter(DELIMITER)
        .from_reader(plain.as_bytes());
    let headers = rdr.headers()?.iter().map(|s| s.to_string()).collect();
    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let row: Row = record.iter().map(|s| Value::from(s.as_ref())).collect();
        rows.push(row);
    }
    Ok((headers, rows))
}

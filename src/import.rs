use crate::error::{Error, Result};
use crate::table::{ColumnType, TableData, TableSchema};
use crate::types::{ExportFormat, Value};
use crate::crypto;
pub fn import_table(
    table_name: &str,
    data: &[u8],
    format: ExportFormat,
    encrypted: bool,
    passphrase: Option<&str>,
) -> Result<TableData> {
    let raw = if encrypted {
        let pass = passphrase.ok_or_else(|| Error::InvalidInput("Passphrase required for decryption".into()))?;
        let decrypted = crypto::decrypt(data, pass)?;
        decrypted.as_bytes().to_vec()
    } else {
        data.to_vec()
    };
    match format {
        ExportFormat::Csv => from_csv(table_name, &raw),
        ExportFormat::Psv => from_psv(table_name, &raw),
        ExportFormat::Json => from_json(table_name, &raw),
        _ => Err(Error::InvalidInput("Unsupported import format".into())),
    }
}
fn from_csv(table_name: &str, data: &[u8]) -> Result<TableData> {
    let mut rdr = csv::Reader::from_reader(data);
    let headers = rdr.headers()?.clone();
    let col_names: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    let schema = TableSchema::new(table_name.to_string(), col_names.into_iter().map(|n| {
        crate::table::ColumnDef { name: n, col_type: ColumnType::Text }
    }).collect());
    let mut rows = vec![];
    for result in rdr.records() {
        let record = result?;
        let row: Vec<Value> = record.iter()
            .map(|s| Value::Text(s.to_string()))
            .collect();
        schema.validate_row(&row)?;
        rows.push(row);
    }
    Ok(TableData { schema, rows })
}
fn from_psv(table_name: &str, data: &[u8]) -> Result<TableData> {
    let text = String::from_utf8(data.to_vec())
        .map_err(|_| Error::InvalidInput("Invalid UTF-8".into()))?;
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return Err(Error::InvalidInput("Empty PSV data".into()));
    }
    let headers: Vec<String> = lines[0].split('|').map(|h| h.trim().to_string()).collect();
    let schema = TableSchema::new(table_name.to_string(), headers.iter().map(|h| {
        crate::table::ColumnDef { name: h.clone(), col_type: ColumnType::Text }
    }).collect());
    let mut rows = vec![];
    for line in &lines[1..] {
        let vals: Vec<Value> = line.split('|')
            .map(|s| Value::Text(s.trim().to_string()))
            .collect();
        schema.validate_row(&vals)?;
        rows.push(vals);
    }
    Ok(TableData { schema, rows })
}
fn from_json(table_name: &str, data: &[u8]) -> Result<TableData> {
    let json: Vec<serde_json::Value> = serde_json::from_slice(data)?;
    if json.is_empty() {
        return Err(Error::InvalidInput("Empty JSON array".into()));
    }
    let first_obj = json[0].as_object()
        .ok_or(Error::InvalidInput("JSON must be array of objects".into()))?;
    let col_names: Vec<String> = first_obj.keys().cloned().collect();
    let schema = TableSchema::new(table_name.to_string(), col_names.iter().map(|n| {
        crate::table::ColumnDef { name: n.clone(), col_type: ColumnType::Text }
    }).collect());
    let mut rows = vec![];
    for obj in &json {
        let obj = obj.as_object()
            .ok_or(Error::InvalidInput("JSON must be array of objects".into()))?;
        let row: Vec<Value> = col_names.iter().map(|key| {
            match obj.get(key) {
                Some(serde_json::Value::String(s)) => Value::Text(s.clone()),
                Some(serde_json::Value::Number(n)) => {
                    if let Some(i) = n.as_i64() {
                        Value::Int(i)
                    } else if let Some(f) = n.as_f64() {
                        Value::Float(f)
                    } else {
                        Value::Text(n.to_string())
                    }
                }
                Some(serde_json::Value::Bool(b)) => Value::Bool(*b),
                _ => Value::Null,
            }
        }).collect();
        schema.validate_row(&row)?;
        rows.push(row);
    }
    Ok(TableData { schema, rows })
}
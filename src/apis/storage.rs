use crate::config;
use crate::error::{NeuxError, Result};
use crate::types::{Row, Value};
use csv::{ReaderBuilder, WriterBuilder};
use std::fs;
pub fn read_table(name: &str) -> Result<(Vec<String>, Vec<Row>)> {
    let path = config::table_path(name)?;
    if !path.exists() {
        return Err(NeuxError::TableNotFound(name.to_string()));
    }
    let data = fs::read_to_string(&path)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(config::delimiter_byte())
        .from_reader(data.as_bytes());
    let headers = rdr.headers()?.iter().map(|s| s.to_string()).collect();
    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let row: Row = record.iter().map(|v| Value::from(v.as_ref())).collect();
        rows.push(row);
    }
    Ok((headers, rows))
}
pub fn write_table(name: &str, headers: &[String], rows: &[Row]) -> Result<()> {
    let path = config::table_path(name)?;
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
        String::from_utf8(buf).map_err(|e| NeuxError::Parse(format!("Invalid UTF-8: {}", e)))?;
    fs::write(path, plain)?;
    Ok(())
}

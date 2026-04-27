use crate::config;
use crate::error::{NeuxDbError, Result};
use crate::types::Row;
use csv::WriterBuilder;
use fs2::FileExt;
use std::fs::{self, File};
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

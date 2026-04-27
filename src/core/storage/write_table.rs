use crate::config;
use crate::error::{NeuxDbError, Result};
use crate::types::Row;
use csv::WriterBuilder;
use fs2::FileExt;
use std::fs::File;
pub fn write_table(name: &str, headers: &[String], rows: &[Row]) -> Result<()> {
    let path = config::table_path(name)?;
    let dir = path.parent().unwrap();
    let tmp_path = dir.join(format!(".{}.tmp", name));
    let file = File::create(&tmp_path)?;
    file.lock_exclusive()
        .map_err(|e| NeuxDbError::Lock(format!("Failed to acquire exclusive lock: {}", e)))?;
    {
        let mut wtr = WriterBuilder::new()
            .delimiter(config::delimiter_byte())
            .from_writer(&file);
        wtr.write_record(headers)?;
        for row in rows {
            let str_row: Vec<String> = row.iter().map(|v| v.to_string()).collect();
            wtr.write_record(&str_row)?;
        }
        wtr.flush()?;
    }
    std::fs::rename(&tmp_path, &path)?;
    Ok(())
}

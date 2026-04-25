use crate::config::{table_path, DELIMITER};
use crate::error::Result;
use crate::types::Row;
use csv::WriterBuilder;
use std::fs;
pub fn write_table(name: &str, headers: &[String], rows: &[Row]) -> Result<()> {
    let path = table_path(name);
    let mut buf = Vec::new();
    {
        let mut wtr = WriterBuilder::new()
            .delimiter(DELIMITER)
            .from_writer(&mut buf);
        wtr.write_record(headers)?;
        for row in rows {
            let str_row: Vec<String> = row.iter().map(|v| v.to_string()).collect();
            wtr.write_record(&str_row)?;
        }
        wtr.flush()?;
    }
    let plain = String::from_utf8(buf).unwrap();
    fs::write(path, plain)?;
    Ok(())
}

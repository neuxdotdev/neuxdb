use super::save_chema::save_schema;
use crate::config;
use crate::error::{NeuxDbError, Result};
use crate::types::TableSchema;
use csv::WriterBuilder;
use fs2::FileExt;
use std::fs::File;
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

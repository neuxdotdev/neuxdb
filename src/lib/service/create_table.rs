use crate::config::{table_path, DELIMITER};
use crate::error::{NeuxError, Result};
use csv::WriterBuilder;
use std::path::Path;
pub fn create_table(name: &str, columns: &[String]) -> Result<()> {
    let path = table_path(name)?;
    if Path::new(&path).exists() {
        return Err(NeuxError::TableAlreadyExists(name.to_string()));
    }
    let mut wtr = WriterBuilder::new().delimiter(DELIMITER).from_path(&path)?;
    wtr.write_record(columns)?;
    wtr.flush()?;
    println!("Table '{}' created with columns: {:?}", name, columns);
    Ok(())
}

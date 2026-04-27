use super::DATA_DIR;
use crate::error::Result;
use std::path::PathBuf;
pub fn schema_path(name: &str) -> Result<PathBuf> {
    let safe = super::sanitize_table_name::sanitize_table_name(name)?;
    Ok(PathBuf::from(DATA_DIR).join(format!("{}.schema.json", safe)))
}

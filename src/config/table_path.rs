use super::{DATA_DIR, TABLE_EXT};
use crate::error::Result;
use std::path::PathBuf;
pub fn table_path(name: &str) -> Result<PathBuf> {
    let safe = super::sanitize_table_name::sanitize_table_name(name)?;
    Ok(PathBuf::from(DATA_DIR).join(format!("{}.{}", safe, TABLE_EXT)))
}

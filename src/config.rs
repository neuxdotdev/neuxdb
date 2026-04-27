use crate::error::{NeuxError, Result};
use std::path::{Path, PathBuf};
const DATA_DIR: &str = "data";
const DELIMITER: char = '|';
const TABLE_EXT: &str = "nxdb";
pub fn sanitize_table_name(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err(NeuxError::Parse("Table name cannot be empty".into()));
    }
    let path = Path::new(name);
    if path.components().any(|c| {
        matches!(
            c,
            std::path::Component::ParentDir | std::path::Component::RootDir
        )
    }) {
        return Err(NeuxError::Parse(format!(
            "Path traversal not allowed: '{}'",
            name
        )));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(NeuxError::Parse(format!(
            "Invalid character in table name '{}'",
            name
        )));
    }
    Ok(name.to_string())
}
pub fn table_path(name: &str) -> Result<PathBuf> {
    let safe = sanitize_table_name(name)?;
    Ok(PathBuf::from(DATA_DIR).join(format!("{}.{}", safe, TABLE_EXT)))
}
pub fn ensure_data_dir() -> Result<()> {
    std::fs::create_dir_all(DATA_DIR)?;
    Ok(())
}
pub fn delimiter_byte() -> u8 {
    DELIMITER as u8
}
pub fn delimiter_char() -> char {
    DELIMITER
}

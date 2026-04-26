use crate::error::NeuxError;
use std::path::{Path, PathBuf};
pub const DATA_DIR: &str = "data";
pub const DELIMITER: u8 = b'|';
pub const TABLE_EXT: &str = "nxdb";
pub fn sanitize_table_name(name: &str) -> Result<String, NeuxError> {
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
            "Invalid table name: path traversal not allowed ('{}')",
            name
        )));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(NeuxError::Parse(format!(
            "Invalid table name: use only alphanumeric, underscore, or hyphen ('{}')",
            name
        )));
    }
    Ok(name.to_string())
}
pub fn table_path(name: &str) -> Result<PathBuf, NeuxError> {
    let safe_name = sanitize_table_name(name)?;
    Ok(PathBuf::from(DATA_DIR).join(format!("{}.{}", safe_name, TABLE_EXT)))
}
pub fn ensure_data_dir() -> std::io::Result<()> {
    std::fs::create_dir_all(DATA_DIR)
}

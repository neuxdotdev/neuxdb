use crate::error::NeuxError;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
pub mod global;
pub use global::NeuxConfig;
pub static CONFIG: Lazy<RwLock<NeuxConfig>> = Lazy::new(|| RwLock::new(NeuxConfig::load()));
pub fn init_config() -> crate::error::Result<()> {
    let _unused = CONFIG.read().unwrap();
    Ok(())
}
pub fn ensure_data_dir() -> std::io::Result<()> {
    let config = CONFIG.read().unwrap();
    std::fs::create_dir_all(&config.data_dir)
}
pub fn sanitize_table_name(name: &str) -> crate::error::Result<String> {
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
pub fn table_path(name: &str) -> crate::error::Result<PathBuf> {
    let safe_name = sanitize_table_name(name)?;
    let config = CONFIG.read().unwrap();
    Ok(config
        .data_dir
        .join(format!("{}.{}", safe_name, config.table_ext)))
}
pub fn delimiter_byte() -> u8 {
    CONFIG.read().unwrap().delimiter as u8
}
pub fn delimiter_char() -> char {
    CONFIG.read().unwrap().delimiter
}

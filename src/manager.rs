use crate::config::{get_base_path, set_data_dir};
use crate::error::{DbError, Result};
use std::fs;
use std::sync::RwLock;
lazy_static::lazy_static! {
    static ref CURRENT_DB: RwLock<Option<String>> = RwLock::new(None);
}
pub fn create_database(name: &str) -> Result<()> {
    let path = get_base_path().join(name);
    if path.exists() {
        return Err(DbError::InvalidInput(format!(
            "Database '{}' already exists",
            name
        )));
    }
    fs::create_dir_all(&path)?;
    Ok(())
}
pub fn drop_database(name: &str) -> Result<()> {
    if name == "data" || name == "." {
        return Err(DbError::InvalidInput(
            "Cannot drop root data directory".into(),
        ));
    }
    let path = get_base_path().join(name);
    if !path.exists() {
        return Err(DbError::TableNotFound(format!("Database {}", name)));
    }
    fs::remove_dir_all(path)?;
    Ok(())
}
pub fn use_database(name: &str) -> Result<()> {
    let path = get_base_path().join(name);
    if !path.exists() {
        return Err(DbError::TableNotFound(format!("Database {}", name)));
    }
    let new_path = path.to_string_lossy().to_string();
    set_data_dir(&new_path)?;
    let mut current = CURRENT_DB
        .write()
        .map_err(|e| DbError::Lock(e.to_string()))?;
    *current = Some(name.to_string());
    Ok(())
}
pub fn list_databases() -> Result<Vec<String>> {
    let mut dbs = Vec::new();
    let base = get_base_path();
    if !base.exists() {
        return Ok(dbs);
    }
    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name() {
                dbs.push(name.to_string_lossy().to_string());
            }
        }
    }
    dbs.sort();
    Ok(dbs)
}

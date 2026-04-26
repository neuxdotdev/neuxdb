use crate::error::Result;
use std::fs;
use std::path::PathBuf;
pub fn execute(path: Option<PathBuf>) -> Result<()> {
    let db_path = path.unwrap_or_else(|| PathBuf::from("data"));
    if db_path.exists() {
        println!("Database already exists at: {}", db_path.display());
        return Ok(());
    }
    fs::create_dir_all(&db_path)?;
    println!("Database created at: {}", db_path.display());
    Ok(())
}

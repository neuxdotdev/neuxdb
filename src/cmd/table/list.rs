use crate::config::{DATA_DIR, TABLE_EXT};
use crate::error::Result;
use std::fs;
pub fn execute() -> Result<()> {
    let data_dir = std::path::Path::new(DATA_DIR);
    if !data_dir.exists() {
        println!("No tables found (database not initialized).");
        return Ok(());
    }
    let tables: Vec<_> = fs::read_dir(data_dir)?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some(TABLE_EXT) {
                path.file_stem().map(|s| s.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    if tables.is_empty() {
        println!("No tables found.");
    } else {
        println!("Tables:");
        for name in tables {
            println!("  - {}", name);
        }
    }
    Ok(())
}

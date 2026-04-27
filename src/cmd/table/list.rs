use crate::config::CONFIG;
use crate::error::Result;
use std::fs;
pub fn execute() -> Result<()> {
    let cfg = CONFIG.read().unwrap();
    let data_dir = &cfg.data_dir;
    let ext = &cfg.table_ext;
    if !data_dir.exists() {
        println!("No tables found (database not initialized).");
        return Ok(());
    }
    let tables: Vec<_> = fs::read_dir(data_dir)?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            if path.extension().and_then(|e| e.to_str()) == Some(ext.as_str()) {
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

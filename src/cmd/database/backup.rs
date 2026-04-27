use crate::config::CONFIG;
use crate::error::Result;
use chrono::Local;
use std::fs;
use std::path::PathBuf;
pub fn execute(name: Option<String>) -> Result<()> {
    let cfg = CONFIG.read().unwrap();
    let data_dir = &cfg.data_dir;
    let table_ext = &cfg.table_ext;
    let backup_dir = PathBuf::from("backups");
    fs::create_dir_all(&backup_dir)?;
    let folder_name = match name {
        Some(n) => n,
        None => Local::now().format("%Y%m%d_%H%M%S").to_string(),
    };
    let backup_path = backup_dir.join(folder_name);
    fs::create_dir_all(&backup_path)?;
    if data_dir.exists() {
        for entry in fs::read_dir(data_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some(table_ext.as_str()) {
                let dest = backup_path.join(path.file_name().unwrap());
                fs::copy(&path, &dest)?;
            }
        }
    }
    println!("Backup created at: {}", backup_path.display());
    Ok(())
}

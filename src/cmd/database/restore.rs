use crate::config::CONFIG;
use crate::error::{NeuxError, Result};
use std::fs;
use std::path::PathBuf;
pub fn execute(file: PathBuf) -> Result<()> {
    if !file.exists() || !file.is_dir() {
        return Err(NeuxError::Parse(format!(
            "Backup folder not found: {}",
            file.display()
        )));
    }
    let cfg = CONFIG.read().unwrap();
    let data_dir = &cfg.data_dir;
    let table_ext = &cfg.table_ext;
    for entry in fs::read_dir(&file)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some(table_ext.as_str()) {
            let dest = data_dir.join(path.file_name().unwrap());
            fs::copy(&path, &dest)?;
        }
    }
    println!("Restored from backup: {}", file.display());
    Ok(())
}

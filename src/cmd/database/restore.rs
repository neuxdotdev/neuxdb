use crate::config::{DATA_DIR, TABLE_EXT};
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
    let data_dir = PathBuf::from(DATA_DIR);
    for entry in fs::read_dir(&file)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some(TABLE_EXT) {
            let dest = data_dir.join(path.file_name().unwrap());
            fs::copy(&path, &dest)?;
        }
    }
    println!("Restored from backup: {}", file.display());
    Ok(())
}

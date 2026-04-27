use crate::config;
use crate::error::Result;
pub fn show_tables() -> Result<Vec<String>> {
    let dir = config::get_data_dir();
    let mut tables = std::collections::HashSet::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "nxdb" {
                        if let Some(stem) = path.file_stem() {
                            tables.insert(stem.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }
    let mut v: Vec<_> = tables.into_iter().collect();
    v.sort();
    Ok(v)
}

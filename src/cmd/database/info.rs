use crate::config::CONFIG;
use crate::error::Result;
use std::fs;
pub fn execute() -> Result<()> {
    let cfg = CONFIG.read().unwrap();
    let data_dir = &cfg.data_dir;
    let ext = &cfg.table_ext;
    let tables = if data_dir.exists() {
        fs::read_dir(data_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some(ext.as_str()))
            .count()
    } else {
        0
    };
    println!("NeuxDB Information:");
    println!("  Data Directory:   {:?}", data_dir);
    println!("  Table extension:  .{}", ext);
    println!("  Delimiter:        '{}'", cfg.delimiter);
    println!("  Number of tables: {}", tables);
    Ok(())
}

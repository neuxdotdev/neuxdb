use crate::config::{DATA_DIR, DELIMITER, TABLE_EXT};
use crate::error::Result;
use std::fs;
pub fn execute() -> Result<()> {
    let data_dir = std::path::Path::new(DATA_DIR);
    let tables = if data_dir.exists() {
        fs::read_dir(data_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some(TABLE_EXT))
            .count()
    } else {
        0
    };
    println!("NeuxDB Information:");
    println!("  Data Directory:   {}", DATA_DIR);
    println!("  Table extension:  .{}", TABLE_EXT);
    println!("  Delimiter:        '{}'", DELIMITER as char);
    println!("  Number of tables: {}", tables);
    Ok(())
}

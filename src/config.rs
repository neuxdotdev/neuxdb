use std::path::PathBuf;
pub const DATA_DIR: &str = "data";
pub const DELIMITER: u8 = b'|';
pub const TABLE_EXT: &str = "ndbx";
pub fn table_path(name: &str) -> PathBuf {
    PathBuf::from(DATA_DIR).join(format!("{}.{}", name, TABLE_EXT))
}
pub fn ensure_data_dir() -> std::io::Result<()> {
    std::fs::create_dir_all(DATA_DIR)
}

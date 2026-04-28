use std::path::PathBuf;
use std::sync::RwLock;
lazy_static::lazy_static! {
    static ref CONFIG: RwLock<NeuxConfig> = RwLock::new(NeuxConfig::default());
}
#[derive(Debug, Clone)]
pub struct NeuxConfig {
    pub data_dir: PathBuf,
    pub delimiter: u8,
    pub table_ext: String,
    pub schema_ext: String,
}
impl Default for NeuxConfig {
    fn default() -> Self {
        let dir = std::env::var("NEUXDB_DATA_DIR").unwrap_or_else(|_| "data".to_string());
        Self {
            data_dir: PathBuf::from(dir),
            delimiter: b'|',
            table_ext: "nxdb".to_string(),
            schema_ext: "schema.json".to_string(),
        }
    }
}
pub fn set_data_dir(path: &str) -> crate::error::Result<()> {
    let mut config = CONFIG
        .write()
        .map_err(|e| crate::error::DbError::Lock(e.to_string()))?;
    config.data_dir = PathBuf::from(path);
    std::fs::create_dir_all(&config.data_dir)?;
    Ok(())
}
pub fn get_base_path() -> PathBuf {
    CONFIG.read().unwrap().data_dir.clone()
}
pub fn get_delimiter() -> u8 {
    CONFIG.read().unwrap().delimiter
}
pub fn get_table_ext() -> String {
    CONFIG.read().unwrap().table_ext.clone()
}
pub fn table_path(name: &str) -> PathBuf {
    get_base_path().join(format!("{}.{}", name, CONFIG.read().unwrap().table_ext))
}
pub fn schema_path(name: &str) -> PathBuf {
    get_base_path().join(format!("{}.{}", name, CONFIG.read().unwrap().schema_ext))
}

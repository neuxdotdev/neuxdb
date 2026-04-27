use crate::error::{NeuxError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuxConfig {
    pub data_dir: PathBuf,
    pub delimiter: char,
    pub table_ext: String,
    pub encryption: EncryptionConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    pub enabled: bool,
    pub passphrase: Option<String>,
    pub key_file: Option<PathBuf>,
}
impl Default for NeuxConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("data"),
            delimiter: '|',
            table_ext: "nxdb".to_string(),
            encryption: EncryptionConfig::default(),
        }
    }
}
impl NeuxConfig {
    pub fn load() -> Self {
        let path = Self::config_file();
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }
        Self::default()
    }
    pub fn save(&self) -> Result<()> {
        let path = Self::config_file();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| NeuxError::Parse(e.to_string()))?;
        fs::write(path, content)?;
        Ok(())
    }
    pub fn config_file() -> PathBuf {
        if let Some(proj_dir) = std::env::var_os("NEUXDB_CONFIG_DIR") {
            return PathBuf::from(proj_dir).join("config.toml");
        }
        if let Some(config_dir) = dirs::config_dir() {
            return config_dir.join("neuxdb").join("config.toml");
        }
        PathBuf::from(".neuxdb.toml")
    }
}

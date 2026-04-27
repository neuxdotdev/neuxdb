use crate::config::CONFIG;
use crate::error::{NeuxError, Result};
use crate::types::commands::ConfigCommands;
use std::path::PathBuf;
pub fn execute(cmd: ConfigCommands) -> Result<()> {
    match cmd {
        ConfigCommands::Show => show(),
        ConfigCommands::Get { key } => get(&key),
        ConfigCommands::Set { key, value } => set(&key, &value),
        ConfigCommands::Reset { key } => reset(&key),
    }
}
fn show() -> Result<()> {
    let cfg = CONFIG.read().unwrap();
    println!("Current NeuxDB configuration:");
    println!("  data_dir:      {:?}", cfg.data_dir);
    println!("  delimiter:     '{}'", cfg.delimiter);
    println!("  table_ext:     {}", cfg.table_ext);
    println!("  encryption.enabled: {}", cfg.encryption.enabled);
    Ok(())
}
fn get(key: &str) -> Result<()> {
    let cfg = CONFIG.read().unwrap();
    match key {
        "data_dir" => println!("{:?}", cfg.data_dir),
        "delimiter" => println!("{}", cfg.delimiter),
        "table_ext" => println!("{}", cfg.table_ext),
        "encryption.enabled" => println!("{}", cfg.encryption.enabled),
        _ => return Err(NeuxError::Parse(format!("Unknown config key '{}'", key))),
    }
    Ok(())
}
fn set(key: &str, value: &str) -> Result<()> {
    let mut cfg = CONFIG.write().unwrap();
    match key {
        "data_dir" => cfg.data_dir = PathBuf::from(value),
        "delimiter" => {
            let c = value
                .chars()
                .next()
                .ok_or_else(|| NeuxError::Parse("Empty delimiter".into()))?;
            cfg.delimiter = c;
        }
        "table_ext" => cfg.table_ext = value.to_string(),
        "encryption.enabled" => {
            let b = value
                .parse::<bool>()
                .map_err(|_| NeuxError::Parse("Must be true or false".into()))?;
            cfg.encryption.enabled = b;
        }
        _ => return Err(NeuxError::Parse(format!("Unknown config key '{}'", key))),
    }
    cfg.save()?;
    println!("Config updated. Some changes may require restart.");
    Ok(())
}
fn reset(key: &str) -> Result<()> {
    let mut cfg = CONFIG.write().unwrap();
    let default = crate::config::NeuxConfig::default();
    match key {
        "data_dir" => cfg.data_dir = default.data_dir,
        "delimiter" => cfg.delimiter = default.delimiter,
        "table_ext" => cfg.table_ext = default.table_ext,
        "encryption.enabled" => cfg.encryption.enabled = default.encryption.enabled,
        "all" => *cfg = default,
        _ => return Err(NeuxError::Parse(format!("Unknown config key '{}'", key))),
    }
    cfg.save()?;
    println!("Config reset to default.");
    Ok(())
}

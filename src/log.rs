use crate::config;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub action: String,
    pub table: String,
    pub details: Option<String>,
}
impl LogEntry {
    pub fn new(action: &str, table: &str, details: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            timestamp,
            action: action.to_string(),
            table: table.to_string(),
            details,
        }
    }
}
pub fn trim_logs_if_needed(logs: &mut Vec<LogEntry>, db_path: &Path) -> Result<()> {
    if logs.len() <= config::MAX_LOG_ENTRIES {
        return Ok(());
    }
    let drain_up_to = logs.len() - config::MAX_LOG_ENTRIES;
    let old_logs: Vec<LogEntry> = logs.drain(0..drain_up_to).collect();
    let archive_path = db_path.with_extension("ndbx.log");
    let json = serde_json::to_string(&old_logs)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(archive_path)?;
    writeln!(file, "--- Archived at {} ---", timestamp_now())?;
    file.write_all(json.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}
fn timestamp_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

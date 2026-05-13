use crate::config;
use serde::{Deserialize, Serialize};
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
pub fn trim_logs_if_needed(logs: &mut Vec<LogEntry>) {
    if logs.len() > config::MAX_LOG_ENTRIES {
        let drain_up_to = logs.len() - config::MAX_LOG_ENTRIES;
        logs.drain(0..drain_up_to);
    }
}

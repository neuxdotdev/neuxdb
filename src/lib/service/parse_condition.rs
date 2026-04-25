use crate::error::{NeuxError, Result};
use crate::types::Value;
pub fn parse_condition(cond: &str) -> Result<Option<(String, Value)>> {
    let trimmed = cond.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let eq = trimmed
        .find('=')
        .ok_or_else(|| NeuxError::Parse("Missing '=' in condition".into()))?;
    let col = trimmed[..eq].to_string();
    let raw_val = trimmed[eq + 1..].trim();
    let val_str = if raw_val.starts_with('\'') && raw_val.ends_with('\'') {
        &raw_val[1..raw_val.len() - 1]
    } else {
        raw_val
    };
    let val = Value::from(val_str);
    Ok(Some((col, val)))
}

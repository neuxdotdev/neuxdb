use crate::error::{NeuxDbError, Result};
use crate::types::Value;
pub fn parse_assignment(assign: &str) -> Result<(String, Value)> {
    let eq = assign
        .find('=')
        .ok_or_else(|| NeuxDbError::Parse("Missing '=' in assignment".into()))?;
    let col = assign[..eq].to_string();
    let raw_val = assign[eq + 1..].trim();
    let val_str = if raw_val.starts_with('\'') && raw_val.ends_with('\'') {
        &raw_val[1..raw_val.len() - 1]
    } else {
        raw_val
    };
    Ok((col, Value::from(val_str)))
}

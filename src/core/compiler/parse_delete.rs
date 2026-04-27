use super::unquote::unquote;
use crate::core::syntax::Statement;
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
pub(super) fn parse_delete(parts: &[&str]) -> Result<Statement> {
    if parts.len() < 6 || parts[1] != "from" || parts[3] != "where" {
        return Err(NeuxDbError::Parse(
            "Syntax: DELETE FROM table WHERE col=value".into(),
        ));
    }
    let table = parts[2].to_string();
    let cond_pair = parts[4];
    let eq_pos = cond_pair
        .find('=')
        .ok_or_else(|| NeuxDbError::Parse("Invalid condition, expected col=value".into()))?;
    let col = cond_pair[..eq_pos].to_string();
    let raw_val = &cond_pair[eq_pos + 1..];
    let val = Value::from(unquote(raw_val).as_str());
    Ok(Statement::Delete {
        table,
        condition: (col, val),
    })
}

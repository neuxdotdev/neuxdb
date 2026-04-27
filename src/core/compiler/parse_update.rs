use super::unquote::unquote;
use crate::core::syntax::Statement;
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
pub(super) fn parse_update(parts: &[&str]) -> Result<Statement> {
    if parts.len() < 8 || parts[2] != "set" || parts[4] != "where" {
        return Err(NeuxDbError::Parse(
            "Syntax: UPDATE table SET col=value WHERE col=value".into(),
        ));
    }
    let table = parts[1].to_string();
    let set_pair = parts[3];
    let eq_set = set_pair
        .find('=')
        .ok_or_else(|| NeuxDbError::Parse("Missing '=' in SET".into()))?;
    let set_col = set_pair[..eq_set].to_string();
    let raw_set_val = &set_pair[eq_set + 1..];
    let set_val = Value::from(unquote(raw_set_val).as_str());
    let cond_pair = parts[5];
    let eq_cond = cond_pair
        .find('=')
        .ok_or_else(|| NeuxDbError::Parse("Missing '=' in WHERE".into()))?;
    let cond_col = cond_pair[..eq_cond].to_string();
    let raw_cond_val = &cond_pair[eq_cond + 1..];
    let cond_val = Value::from(unquote(raw_cond_val).as_str());
    Ok(Statement::Update {
        table,
        set_col,
        set_val,
        condition: (cond_col, cond_val),
    })
}

use super::unquote::unquote;
use crate::core::syntax::Statement;
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
pub(super) fn parse_select(parts: &[&str]) -> Result<Statement> {
    if parts.len() < 4 || parts[2] != "from" {
        return Err(NeuxDbError::Parse(
            "Syntax: SELECT columns FROM table [WHERE col=value]".into(),
        ));
    }
    let columns = if parts[1] == "*" {
        vec!["*".to_string()]
    } else {
        parts[1].split(',').map(|s| s.trim().to_string()).collect()
    };
    let table = parts[3].to_string();
    let mut condition = None;
    if parts.len() > 4 && parts[4] == "where" && parts.len() >= 6 {
        let cond_str = parts[5];
        let eq_pos = cond_str
            .find('=')
            .ok_or_else(|| NeuxDbError::Parse("Invalid condition".into()))?;
        let col = cond_str[..eq_pos].to_string();
        let raw_val = &cond_str[eq_pos + 1..];
        let val = Value::from(unquote(raw_val).as_str());
        condition = Some((col, val));
    }
    Ok(Statement::Select {
        columns,
        table,
        condition,
    })
}

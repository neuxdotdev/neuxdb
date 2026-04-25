use crate::lib::sintax::Statement;
use crate::types::Value;
use anyhow::{bail, Result};
pub(crate) fn parse_delete(parts: &[&str], unquote: &dyn Fn(&str) -> String) -> Result<Statement> {
    if parts.len() < 6 || parts[1] != "from" || parts[3] != "where" {
        bail!("Syntax: DELETE FROM table WHERE col=value");
    }
    let table = parts[2].to_string();
    let cond_pair = parts[4];
    let eq_pos = cond_pair
        .find('=')
        .ok_or_else(|| anyhow::anyhow!("Invalid condition, expected col=value"))?;
    let col = cond_pair[..eq_pos].to_string();
    let raw_val = &cond_pair[eq_pos + 1..];
    let val = Value::from(unquote(raw_val).as_str());
    Ok(Statement::Delete {
        table,
        condition: (col, val),
    })
}

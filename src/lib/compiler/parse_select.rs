use crate::lib::sintax::Statement;
use crate::types::Value;
use anyhow::{bail, Result};
pub(crate) fn parse_select(parts: &[&str], unquote: &dyn Fn(&str) -> String) -> Result<Statement> {
    if parts.len() < 4 || parts[2] != "from" {
        bail!("Syntax: SELECT columns FROM table [WHERE col=value]");
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
            .ok_or_else(|| anyhow::anyhow!("Invalid condition"))?;
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

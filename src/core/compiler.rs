use crate::core::syntax::{split_quoted, Statement};
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
fn unquote(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.starts_with('\'') && trimmed.ends_with('\'') {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}
pub fn parse(sql: &str) -> Result<Statement> {
    let parts: Vec<&str> = sql.split_whitespace().collect();
    if parts.is_empty() {
        return Err(NeuxDbError::Parse("Empty query".into()));
    }
    match parts[0] {
        "create" => parse_create(&parts),
        "insert" => parse_insert(&parts),
        "select" => parse_select(&parts),
        "update" => parse_update(&parts),
        "delete" => parse_delete(&parts),
        _ => Err(NeuxDbError::Parse(format!("Unknown command: {}", parts[0]))),
    }
}
fn parse_create(parts: &[&str]) -> Result<Statement> {
    if parts.len() < 4 || parts[1] != "table" {
        return Err(NeuxDbError::Parse(
            "Syntax: CREATE TABLE table_name (col1, col2, ...)".into(),
        ));
    }
    let name = parts[2].to_string();
    let rest = parts[3..].join(" ");
    let open = rest
        .find('(')
        .ok_or_else(|| NeuxDbError::Parse("Missing '('".into()))?;
    let close = rest
        .rfind(')')
        .ok_or_else(|| NeuxDbError::Parse("Missing ')'".into()))?;
    let cols_str = &rest[open + 1..close];
    let columns = cols_str.split(',').map(|s| s.trim().to_string()).collect();
    Ok(Statement::CreateTable { name, columns })
}
fn parse_insert(parts: &[&str]) -> Result<Statement> {
    if parts.len() < 5 || parts[1] != "into" || parts[3] != "values" {
        return Err(NeuxDbError::Parse(
            "Syntax: INSERT INTO table VALUES (val1|val2|...)".into(),
        ));
    }
    let table = parts[2].to_string();
    let values_part = parts[4..].join(" ");
    let open = values_part
        .find('(')
        .ok_or_else(|| NeuxDbError::Parse("Missing '('".into()))?;
    let close = values_part
        .rfind(')')
        .ok_or_else(|| NeuxDbError::Parse("Missing ')'".into()))?;
    let vals_str = &values_part[open + 1..close];
    let raw_values = split_quoted(vals_str, '|');
    let values = raw_values
        .into_iter()
        .map(|s| Value::from(unquote(&s).as_str()))
        .collect();
    Ok(Statement::Insert { table, values })
}
fn parse_select(parts: &[&str]) -> Result<Statement> {
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
fn parse_update(parts: &[&str]) -> Result<Statement> {
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
fn parse_delete(parts: &[&str]) -> Result<Statement> {
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

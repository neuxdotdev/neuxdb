use super::unquote::unquote;
use crate::core::syntax::{split_quoted, Statement};
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
pub(super) fn parse_insert(parts: &[&str]) -> Result<Statement> {
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

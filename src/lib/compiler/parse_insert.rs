use crate::lib::sintax::Statement;
use crate::types::Value;
use anyhow::{bail, Result};
pub(crate) fn parse_insert(parts: &[&str], unquote: &dyn Fn(&str) -> String) -> Result<Statement> {
    if parts.len() < 5 || parts[1] != "into" || parts[3] != "values" {
        bail!("Syntax: INSERT INTO table VALUES (val1|val2|...)");
    }
    let table = parts[2].to_string();
    let values_part = parts[4..].join(" ");
    let open = values_part
        .find('(')
        .ok_or_else(|| anyhow::anyhow!("Missing '('"))?;
    let close = values_part
        .rfind(')')
        .ok_or_else(|| anyhow::anyhow!("Missing ')'"))?;
    let vals_str = &values_part[open + 1..close];
    let values = vals_str
        .split('|')
        .map(|s| Value::from(unquote(s.trim()).as_str()))
        .collect();
    Ok(Statement::Insert { table, values })
}

use crate::lib::sintax::Statement;
use anyhow::{bail, Result};
pub(crate) fn parse_create(parts: &[&str]) -> Result<Statement> {
    if parts.len() < 4 || parts[1] != "table" {
        bail!("Syntax: CREATE TABLE table_name (col1, col2, ...)");
    }
    let name = parts[2].to_string();
    let rest = parts[3..].join(" ");
    let open = rest
        .find('(')
        .ok_or_else(|| anyhow::anyhow!("Missing '('"))?;
    let close = rest
        .rfind(')')
        .ok_or_else(|| anyhow::anyhow!("Missing ')'"))?;
    let cols_str = &rest[open + 1..close];
    let columns = cols_str.split(',').map(|s| s.trim().to_string()).collect();
    Ok(Statement::CreateTable { name, columns })
}

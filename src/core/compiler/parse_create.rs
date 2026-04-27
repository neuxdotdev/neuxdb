use crate::core::syntax::Statement;
use crate::error::{NeuxDbError, Result};
pub(super) fn parse_create(parts: &[&str]) -> Result<Statement> {
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

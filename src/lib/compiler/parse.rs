use super::{
    parse_create::parse_create, parse_delete::parse_delete, parse_insert::parse_insert,
    parse_select::parse_select, parse_update::parse_update,
};
use crate::lib::sintax::Statement;
use anyhow::{bail, Result};
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
        bail!("Empty query");
    }
    match parts[0] {
        "create" => parse_create(&parts),
        "insert" => parse_insert(&parts, &unquote),
        "select" => parse_select(&parts, &unquote),
        "update" => parse_update(&parts, &unquote),
        "delete" => parse_delete(&parts, &unquote),
        _ => bail!("Unknown command: {}", parts[0]),
    }
}

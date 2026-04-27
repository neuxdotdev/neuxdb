use super::parse_create::parse_create;
use super::parse_delete::parse_delete;
use super::parse_insert::parse_insert;
use super::parse_select::parse_select;
use super::parse_update::parse_update;
use crate::core::syntax::Statement;
use crate::error::{NeuxDbError, Result};
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

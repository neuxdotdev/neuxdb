use super::parse_create::parse_create;
use super::parse_delete::parse_delete;
use super::parse_insert::parse_insert;
use super::parse_select::parse_select;
use super::parse_update::parse_update;
use super::tokenizer::tokenize;
use crate::core::syntax::Statement;
use crate::error::{NeuxDbError, Result};
pub fn parse(sql: &str) -> Result<Statement> {
    let tokens = tokenize(sql);
    let mut iter = tokens.iter().peekable();
    match iter.peek().map(|s| s.as_str()) {
        Some("create") => {
            iter.next();
            parse_create(&mut iter)
        }
        Some("insert") => {
            iter.next();
            parse_insert(&mut iter)
        }
        Some("select") => {
            iter.next();
            parse_select(&mut iter)
        }
        Some("update") => {
            iter.next();
            parse_update(&mut iter)
        }
        Some("delete") => {
            iter.next();
            parse_delete(&mut iter)
        }
        Some(other) => Err(NeuxDbError::Parse(format!("Unknown command: {}", other))),
        None => Err(NeuxDbError::Parse("Empty query".into())),
    }
}

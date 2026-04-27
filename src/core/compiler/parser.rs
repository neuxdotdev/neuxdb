use super::parse_create::parse_create;
use super::parse_delete::parse_delete;
use super::parse_insert::parse_insert;
use super::parse_select::parse_select;
use super::parse_update::parse_update;
use super::tokenizer::tokenize;
use crate::core::syntax::{ComparisonOp, Statement, WhereClause};
use crate::error::NeuxDbError;
use crate::types::Value;
use std::iter::Peekable;
use std::slice::Iter;
pub fn parse(sql: &str) -> Result<Statement, NeuxDbError> {
    let tokens = tokenize(sql);
    let mut iter = tokens.iter().peekable();
    let command = iter.peek().map(|s| s.to_lowercase());
    if command.as_deref() == Some("create") {
        iter.next();
        parse_create(&mut iter)
    } else if command.as_deref() == Some("drop") {
        iter.next();
        parse_drop(&mut iter)
    } else if command.as_deref() == Some("show") {
        iter.next();
        parse_show(&mut iter)
    } else if command.as_deref() == Some("insert") {
        iter.next();
        parse_insert(&mut iter)
    } else if command.as_deref() == Some("select") {
        iter.next();
        parse_select(&mut iter)
    } else if command.as_deref() == Some("update") {
        iter.next();
        parse_update(&mut iter)
    } else if command.as_deref() == Some("delete") {
        iter.next();
        parse_delete(&mut iter)
    } else if let Some(other) = command {
        Err(NeuxDbError::Parse(format!(
            "Unknown command '{}'. Expected CREATE, DROP, SHOW, INSERT, SELECT, UPDATE, or DELETE",
            other
        )))
    } else {
        Err(NeuxDbError::Parse("Empty query".into()))
    }
}
fn parse_drop(iter: &mut Peekable<Iter<String>>) -> Result<Statement, NeuxDbError> {
    match iter.next() {
        Some(t) if t.to_lowercase() == "table" => {}
        _ => return Err(NeuxDbError::Parse("Expected 'TABLE' after DROP".into())),
    }
    let name = next_identifier(iter, "table name")?;
    Ok(Statement::DropTable { name })
}
fn parse_show(iter: &mut Peekable<Iter<String>>) -> Result<Statement, NeuxDbError> {
    match iter.next() {
        Some(t) if t.to_lowercase() == "tables" => {}
        _ => return Err(NeuxDbError::Parse("Expected 'TABLES' after SHOW".into())),
    }
    Ok(Statement::ShowTables)
}
pub(super) fn next_identifier<'a>(
    iter: &mut Peekable<Iter<'a, String>>,
    expected: &str,
) -> Result<String, NeuxDbError> {
    match iter.next() {
        Some(t) => Ok(t.clone()),
        None => Err(NeuxDbError::Parse(format!("Missing {}", expected))),
    }
}
pub(super) fn parse_where_clause(
    iter: &mut Peekable<Iter<String>>,
) -> Result<WhereClause, NeuxDbError> {
    let left = parse_condition(iter)?;
    let next = iter.peek().map(|s| s.to_lowercase());
    if next.as_deref() == Some("and") {
        iter.next();
        let right = parse_where_clause(iter)?;
        Ok(WhereClause::And(Box::new(left), Box::new(right)))
    } else if next.as_deref() == Some("or") {
        iter.next();
        let right = parse_where_clause(iter)?;
        Ok(WhereClause::Or(Box::new(left), Box::new(right)))
    } else {
        Ok(left)
    }
}
fn parse_condition(iter: &mut Peekable<Iter<String>>) -> Result<WhereClause, NeuxDbError> {
    let column = next_identifier(iter, "column name")?;
    let op = match iter.next() {
        Some(op) => op.clone(),
        None => return Err(NeuxDbError::Parse("Missing operator after column".into())),
    };
    let operator = match op.as_str() {
        "=" => ComparisonOp::Eq,
        "!=" | "<>" => ComparisonOp::Ne,
        "<" => ComparisonOp::Lt,
        ">" => ComparisonOp::Gt,
        "<=" => ComparisonOp::Le,
        ">=" => ComparisonOp::Ge,
        s if s.to_lowercase() == "like" => ComparisonOp::Like,
        _ => return Err(NeuxDbError::Parse(format!("Unknown operator '{}'", op))),
    };
    let raw_val = next_identifier(iter, "value")?;
    let unquoted = super::unquote::unquote(&raw_val);
    let value = Value::from(unquoted.as_str());
    Ok(WhereClause::Condition {
        column,
        operator,
        value,
    })
}

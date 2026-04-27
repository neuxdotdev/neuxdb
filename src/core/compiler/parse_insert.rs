use super::unquote::unquote;
use crate::core::syntax::{split_quoted, Statement};
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
use std::iter::Peekable;
use std::slice::Iter;
pub(super) fn parse_insert(iter: &mut Peekable<Iter<String>>) -> Result<Statement> {
    match iter.next() {
        Some(t) if *t == "into" => {}
        _ => return Err(NeuxDbError::Parse("Expected 'into' after INSERT".into())),
    }
    let table = match iter.next() {
        Some(t) => t.clone(),
        None => return Err(NeuxDbError::Parse("Missing table name".into())),
    };
    match iter.next() {
        Some(t) if *t == "values" => {}
        _ => {
            return Err(NeuxDbError::Parse(
                "Expected 'values' after table name".into(),
            ))
        }
    }
    match iter.next() {
        Some(t) if *t == "(" => {}
        _ => return Err(NeuxDbError::Parse("Missing '(' after VALUES".into())),
    }
    let mut value_tokens = Vec::new();
    loop {
        match iter.next() {
            Some(t) if *t == ")" => break,
            Some(t) => value_tokens.push(t.clone()),
            None => return Err(NeuxDbError::Parse("Missing ')'".into())),
        }
    }
    let joined = value_tokens.join("");
    let raw_values = split_quoted(&joined, '|');
    let values: Vec<Value> = raw_values
        .into_iter()
        .map(|s| Value::from(unquote(&s).as_str()))
        .collect();
    Ok(Statement::Insert { table, values })
}

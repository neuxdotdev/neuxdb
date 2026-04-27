use super::unquote::unquote;
use crate::core::syntax::Statement;
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
use std::iter::Peekable;
use std::slice::Iter;
pub(super) fn parse_update(iter: &mut Peekable<Iter<String>>) -> Result<Statement> {
    let table = match iter.next() {
        Some(t) => t.clone(),
        None => return Err(NeuxDbError::Parse("Missing table name in UPDATE".into())),
    };
    match iter.next() {
        Some(s) if *s == "set" => {}
        _ => return Err(NeuxDbError::Parse("Expected 'set' in UPDATE".into())),
    }
    let set_col = match iter.next() {
        Some(c) => c.clone(),
        None => return Err(NeuxDbError::Parse("Missing column in SET".into())),
    };
    match iter.next() {
        Some(op) if *op == "=" => {}
        _ => return Err(NeuxDbError::Parse("Missing '=' in SET".into())),
    }
    let raw_set_val = match iter.next() {
        Some(v) => v.clone(),
        None => return Err(NeuxDbError::Parse("Missing value in SET".into())),
    };
    let set_val = Value::from(unquote(&raw_set_val).as_str());
    match iter.next() {
        Some(w) if *w == "where" => {}
        _ => return Err(NeuxDbError::Parse("Missing 'where' in UPDATE".into())),
    }
    let cond_col = match iter.next() {
        Some(c) => c.clone(),
        None => return Err(NeuxDbError::Parse("Missing column in WHERE".into())),
    };
    match iter.next() {
        Some(op) if *op == "=" => {}
        _ => return Err(NeuxDbError::Parse("Missing '=' in WHERE".into())),
    }
    let raw_cond_val = match iter.next() {
        Some(v) => v.clone(),
        None => return Err(NeuxDbError::Parse("Missing value in WHERE".into())),
    };
    let cond_val = Value::from(unquote(&raw_cond_val).as_str());
    Ok(Statement::Update {
        table,
        set_col,
        set_val,
        condition: (cond_col, cond_val),
    })
}

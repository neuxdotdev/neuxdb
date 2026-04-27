use super::unquote::unquote;
use crate::core::syntax::Statement;
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
use std::iter::Peekable;
use std::slice::Iter;
pub(super) fn parse_delete(iter: &mut Peekable<Iter<String>>) -> Result<Statement> {
    match iter.next() {
        Some(f) if *f == "from" => {}
        _ => return Err(NeuxDbError::Parse("Expected 'from' in DELETE".into())),
    }
    let table = match iter.next() {
        Some(t) => t.clone(),
        None => return Err(NeuxDbError::Parse("Missing table name in DELETE".into())),
    };
    match iter.next() {
        Some(w) if *w == "where" => {}
        _ => return Err(NeuxDbError::Parse("Missing 'where' in DELETE".into())),
    }
    let col = match iter.next() {
        Some(c) => c.clone(),
        None => return Err(NeuxDbError::Parse("Missing column in WHERE".into())),
    };
    match iter.next() {
        Some(op) if *op == "=" => {}
        _ => return Err(NeuxDbError::Parse("Missing '=' in WHERE".into())),
    }
    let raw_val = match iter.next() {
        Some(v) => v.clone(),
        None => return Err(NeuxDbError::Parse("Missing value in WHERE".into())),
    };
    let val = Value::from(unquote(&raw_val).as_str());
    Ok(Statement::Delete {
        table,
        condition: (col, val),
    })
}

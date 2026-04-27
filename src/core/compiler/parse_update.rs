use super::unquote::unquote;
use crate::core::syntax::Statement;
use crate::error::NeuxDbError;
use crate::types::Value;
use std::iter::Peekable;
use std::slice::Iter;
pub(super) fn parse_update(iter: &mut Peekable<Iter<String>>) -> Result<Statement, NeuxDbError> {
    let table = match iter.next() {
        Some(t) => t.clone(),
        None => return Err(NeuxDbError::Parse("Missing table name in UPDATE".into())),
    };
    match iter.next() {
        Some(s) if s.to_lowercase() == "set" => {}
        _ => return Err(NeuxDbError::Parse("Expected 'SET' in UPDATE".into())),
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
        Some(w) if w.to_lowercase() == "where" => {}
        _ => return Err(NeuxDbError::Parse("Expected 'WHERE' in UPDATE".into())),
    }
    let condition = super::parser::parse_where_clause(iter)?;
    Ok(Statement::Update {
        table,
        set_col,
        set_val,
        condition,
    })
}

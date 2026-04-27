use super::unquote::unquote;
use crate::core::syntax::Statement;
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
use std::iter::Peekable;
use std::slice::Iter;
pub(super) fn parse_select(iter: &mut Peekable<Iter<String>>) -> Result<Statement> {
    let mut columns = Vec::new();
    loop {
        match iter.next() {
            Some(col) if *col == "," => continue,
            Some(col) => {
                columns.push(col.clone());
                if iter.peek().map(|s| s.as_str()) == Some("from") {
                    break;
                }
            }
            None => {
                return Err(NeuxDbError::Parse(
                    "Unexpected end in SELECT columns".into(),
                ))
            }
        }
    }
    iter.next();
    let table = match iter.next() {
        Some(t) => t.clone(),
        None => return Err(NeuxDbError::Parse("Missing table name in SELECT".into())),
    };
    let mut condition = None;
    if iter.peek().map(|s| s.as_str()) == Some("where") {
        iter.next();
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
        condition = Some((col, val));
    }
    Ok(Statement::Select {
        columns,
        table,
        condition,
    })
}

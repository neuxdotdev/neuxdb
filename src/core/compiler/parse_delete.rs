use crate::core::syntax::Statement;
use crate::error::NeuxDbError;
use std::iter::Peekable;
use std::slice::Iter;
pub(super) fn parse_delete(iter: &mut Peekable<Iter<String>>) -> Result<Statement, NeuxDbError> {
    match iter.next() {
        Some(f) if f.to_lowercase() == "from" => {}
        _ => return Err(NeuxDbError::Parse("Expected 'FROM' in DELETE".into())),
    }
    let table = match iter.next() {
        Some(t) => t.clone(),
        None => return Err(NeuxDbError::Parse("Missing table name in DELETE".into())),
    };
    match iter.next() {
        Some(w) if w.to_lowercase() == "where" => {}
        _ => return Err(NeuxDbError::Parse("Expected 'WHERE' in DELETE".into())),
    }
    let condition = super::parser::parse_where_clause(iter)?;
    Ok(Statement::Delete { table, condition })
}

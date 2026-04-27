use crate::core::syntax::Statement;
use crate::error::NeuxDbError;
use std::iter::Peekable;
use std::slice::Iter;
pub(super) fn parse_select(iter: &mut Peekable<Iter<String>>) -> Result<Statement, NeuxDbError> {
    let mut columns = Vec::new();
    loop {
        match iter.next() {
            Some(col) if *col == "," => continue,
            Some(col) => {
                columns.push(col.clone());
                if iter.peek().map(|s| s.to_lowercase()) == Some("from".to_string()) {
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
        None => return Err(NeuxDbError::Parse("Missing table name after FROM".into())),
    };
    let mut condition = None;
    let where_lower = iter.peek().map(|s| s.to_lowercase());
    if where_lower == Some("where".to_string()) {
        iter.next();
        condition = Some(super::parser::parse_where_clause(iter)?);
    }
    Ok(Statement::Select {
        columns,
        table,
        condition,
    })
}

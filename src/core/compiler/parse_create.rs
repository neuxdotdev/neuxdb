use crate::core::syntax::Statement;
use crate::error::{NeuxDbError, Result};
use std::iter::Peekable;
use std::slice::Iter;
pub(super) fn parse_create(iter: &mut Peekable<Iter<String>>) -> Result<Statement> {
    match iter.next() {
        Some(t) if *t == "table" => {}
        _ => return Err(NeuxDbError::Parse("Expected 'table' after CREATE".into())),
    }
    let name = match iter.next() {
        Some(n) => n.clone(),
        None => return Err(NeuxDbError::Parse("Missing table name".into())),
    };
    match iter.peek() {
        Some(s) if *s == "(" => {
            iter.next();
        }
        _ => return Err(NeuxDbError::Parse("Missing '(' after table name".into())),
    }
    let mut columns = Vec::new();
    loop {
        match iter.next() {
            Some(col) if *col == ")" => break,
            Some(col) if *col == "," => continue,
            Some(col) => {
                columns.push(col.clone());
            }
            None => return Err(NeuxDbError::Parse("Missing ')'".into())),
        }
    }
    Ok(Statement::CreateTable { name, columns })
}

use crate::error::Result;
use crate::lib::service::{delete_rows, parse_condition};
pub fn execute(table: String, r#where: String) -> Result<()> {
    let (col, val) = parse_condition(&r#where)?
        .ok_or_else(|| crate::error::NeuxError::Parse("WHERE clause required".into()))?;
    delete_rows(&table, &(col, val))
}

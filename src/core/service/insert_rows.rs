use crate::core::storage::{read_table, write_table};
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
pub fn insert_row(table: &str, values: Vec<Value>) -> Result<()> {
    let (headers, mut rows) = read_table(table)?;
    if values.len() != headers.len() {
        return Err(NeuxDbError::ValueCountMismatch {
            expected: headers.len(),
            actual: values.len(),
        });
    }
    rows.push(values);
    write_table(table, &headers, &rows)?;
    Ok(())
}

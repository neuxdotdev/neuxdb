use crate::core::storage::load_schema;
use crate::core::storage::{read_table, write_table};
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
pub fn insert_row(table: &str, values: Vec<Value>) -> Result<()> {
    let schema = load_schema(table)?;
    let (headers, mut rows) = read_table(table)?;
    if values.len() != headers.len() {
        return Err(NeuxDbError::ValueCountMismatch {
            expected: headers.len(),
            actual: values.len(),
        });
    }
    for (i, val) in values.iter().enumerate() {
        schema.validate_value(i, val)?;
    }
    rows.push(values);
    write_table(table, &headers, &rows)?;
    Ok(())
}

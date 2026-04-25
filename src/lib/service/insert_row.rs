use crate::error::{NeuxError, Result};
use crate::lib::storage::{read_table, write_table};
use crate::types::{Row, Value};
pub fn insert_row(table: &str, values: Vec<Value>) -> Result<()> {
    let (headers, mut rows) = read_table(table)?;
    if values.len() != headers.len() {
        return Err(NeuxError::ValueCountMismatch {
            expected: headers.len(),
            actual: values.len(),
        });
    }
    rows.push(values);
    write_table(table, &headers, &rows)?;
    println!("1 row inserted into '{}'", table);
    Ok(())
}

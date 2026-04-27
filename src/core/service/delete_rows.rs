use crate::core::storage::{read_table, write_table};
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
pub fn delete_rows(table: &str, condition: &(String, Value)) -> Result<usize> {
    let (headers, rows) = read_table(table)?;
    let (cond_col, cond_val) = condition;
    let cond_idx = headers
        .iter()
        .position(|c| c == cond_col)
        .ok_or_else(|| NeuxDbError::ColumnNotFound(cond_col.clone(), table.to_string()))?;
    let original_len = rows.len();
    let new_rows: Vec<_> = rows
        .into_iter()
        .filter(|row| row[cond_idx] != *cond_val)
        .collect();
    let deleted = original_len - new_rows.len();
    if deleted > 0 {
        write_table(table, &headers, &new_rows)?;
    }
    Ok(deleted)
}

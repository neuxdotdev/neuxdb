use crate::core::storage::{read_table, write_table};
use crate::error::{NeuxDbError, Result};
use crate::types::Value;
pub fn update_rows(
    table: &str,
    set_col: &str,
    set_val: Value,
    condition: &(String, Value),
) -> Result<usize> {
    let (headers, mut rows) = read_table(table)?;
    let set_idx = headers
        .iter()
        .position(|c| c == set_col)
        .ok_or_else(|| NeuxDbError::ColumnNotFound(set_col.to_string(), table.to_string()))?;
    let (cond_col, cond_val) = condition;
    let cond_idx = headers
        .iter()
        .position(|c| c == cond_col)
        .ok_or_else(|| NeuxDbError::ColumnNotFound(cond_col.clone(), table.to_string()))?;
    let mut updated = 0;
    for row in &mut rows {
        if row[cond_idx] == *cond_val {
            row[set_idx] = set_val.clone();
            updated += 1;
        }
    }
    if updated > 0 {
        write_table(table, &headers, &rows)?;
    }
    Ok(updated)
}

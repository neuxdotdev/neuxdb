use crate::core::storage::read_table;
use crate::error::{NeuxDbError, Result};
use crate::types::{Row, Value};
pub fn select_rows(
    table: &str,
    columns: &[String],
    condition: Option<&(String, Value)>,
) -> Result<Vec<Row>> {
    let (headers, rows) = read_table(table)?;
    let selected_cols = if columns.len() == 1 && columns[0] == "*" {
        headers.clone()
    } else {
        columns.to_vec()
    };
    let col_indices: Vec<usize> = selected_cols
        .iter()
        .map(|c| {
            headers
                .iter()
                .position(|h| h == c)
                .ok_or_else(|| NeuxDbError::ColumnNotFound(c.clone(), table.to_string()))
        })
        .collect::<Result<_>>()?;
    let filter = condition
        .map(|(col, val)| {
            let idx = headers
                .iter()
                .position(|h| h == col)
                .ok_or_else(|| NeuxDbError::ColumnNotFound(col.clone(), table.to_string()))?;
            Ok::<_, NeuxDbError>((idx, val.clone()))
        })
        .transpose()?;
    let result_rows = rows
        .into_iter()
        .filter(|row| {
            if let Some((idx, target)) = &filter {
                &row[*idx] == target
            } else {
                true
            }
        })
        .map(|row| col_indices.iter().map(|&idx| row[idx].clone()).collect())
        .collect();
    Ok(result_rows)
}

use crate::core::storage::{create_table_schema, read_table, write_table};
use crate::error::{NeuxDbError, Result};
use crate::types::{Row, Value};
pub fn create_table(name: &str, columns: &[String]) -> Result<()> {
    create_table_schema(name, columns)
}
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
pub fn parse_condition(cond: &str) -> Result<Option<(String, Value)>> {
    let trimmed = cond.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let eq = trimmed
        .find('=')
        .ok_or_else(|| NeuxDbError::Parse("Missing '=' in condition".into()))?;
    let col = trimmed[..eq].to_string();
    let raw_val = trimmed[eq + 1..].trim();
    let val_str = if raw_val.starts_with('\'') && raw_val.ends_with('\'') {
        &raw_val[1..raw_val.len() - 1]
    } else {
        raw_val
    };
    Ok(Some((col, Value::from(val_str))))
}
pub fn parse_assignment(assign: &str) -> Result<(String, Value)> {
    let eq = assign
        .find('=')
        .ok_or_else(|| NeuxDbError::Parse("Missing '=' in assignment".into()))?;
    let col = assign[..eq].to_string();
    let raw_val = assign[eq + 1..].trim();
    let val_str = if raw_val.starts_with('\'') && raw_val.ends_with('\'') {
        &raw_val[1..raw_val.len() - 1]
    } else {
        raw_val
    };
    Ok((col, Value::from(val_str)))
}
pub fn run_script(path: &str, callback: impl Fn(&str) -> Result<()>) -> Result<()> {
    let file = std::fs::File::open(path)?;
    use std::io::{BufRead, BufReader};
    let reader = BufReader::new(file);
    for (line_no, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        callback(line).map_err(|e| NeuxDbError::Parse(format!("Line {}: {}", line_no + 1, e)))?;
    }
    Ok(())
}

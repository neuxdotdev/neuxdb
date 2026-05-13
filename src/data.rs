use crate::error::{Error, Result};
use crate::log::LogEntry;
use crate::table::TableData;
use crate::types::Value;
pub fn insert(table: &mut TableData, row: Vec<Value>, logs: &mut Vec<LogEntry>) -> Result<()> {
    table.schema.validate_row(&row)?;
    table.rows.push(row.clone());
    logs.push(LogEntry::new(
        "INSERT",
        &table.schema.name,
        Some(format!("{:?}", row)),
    ));
    Ok(())
}
pub fn select(
    table: &TableData,
    columns: Option<Vec<&str>>,
    filter: Option<&dyn Fn(&[Value]) -> bool>,
) -> Result<Vec<Vec<Value>>> {
    let mut result = Vec::new();
    for row in &table.rows {
        if let Some(pred) = &filter {
            if !pred(row) {
                continue;
            }
        }
        if let Some(cols) = &columns {
            let indices: Vec<usize> = cols
                .iter()
                .map(|c| {
                    table
                        .schema
                        .columns
                        .iter()
                        .position(|col_def| col_def.name == *c)
                        .ok_or_else(|| Error::ColumnNotFound(c.to_string()))
                })
                .collect::<Result<_>>()?;
            let projected: Vec<Value> = indices.iter().map(|&i| row[i].clone()).collect();
            result.push(projected);
        } else {
            result.push(row.clone());
        }
    }
    Ok(result)
}
pub fn update(
    table: &mut TableData,
    filter: &dyn Fn(&[Value]) -> bool,
    set_col: &str,
    new_val: Value,
    logs: &mut Vec<LogEntry>,
) -> Result<usize> {
    let col_idx = table
        .schema
        .columns
        .iter()
        .position(|col_def| col_def.name == set_col)
        .ok_or_else(|| Error::ColumnNotFound(set_col.to_string()))?;
    let col_type = table.schema.columns[col_idx].col_type;
    if !col_type.validate(&new_val) {
        return Err(Error::TypeMismatch {
            column: set_col.to_string(),
            expected: col_type.name().to_string(),
            actual: match &new_val {
                Value::Int(_) => "INT",
                Value::Text(_) => "TEXT",
                Value::Bool(_) => "BOOL",
                Value::Float(_) => "FLOAT",
                Value::Null => "NULL",
            }
            .to_string(),
        });
    }
    let mut count = 0;
    for row in &mut table.rows {
        if filter(row) {
            row[col_idx] = new_val.clone();
            count += 1;
        }
    }
    if count > 0 {
        logs.push(LogEntry::new(
            "UPDATE",
            &table.schema.name,
            Some(format!("Set {} = {:?} on {} rows", set_col, new_val, count)),
        ));
    }
    Ok(count)
}
pub fn delete(
    table: &mut TableData,
    filter: &dyn Fn(&[Value]) -> bool,
    logs: &mut Vec<LogEntry>,
) -> Result<usize> {
    let original = table.rows.len();
    table.rows.retain(|r| !filter(r));
    let removed = original - table.rows.len();
    if removed > 0 {
        logs.push(LogEntry::new(
            "DELETE",
            &table.schema.name,
            Some(format!("Removed {} rows", removed)),
        ));
    }
    Ok(removed)
}

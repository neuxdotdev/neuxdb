use crate::config::DELIMITER;
use crate::error::{NeuxError, Result};
use crate::lib::storage::read_table;
use crate::types::Value;
pub fn select_rows(
    table: &str,
    columns: &[String],
    condition: Option<&(String, Value)>,
) -> Result<()> {
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
                .ok_or_else(|| NeuxError::ColumnNotFound(c.clone(), table.to_string()))
        })
        .collect::<Result<_>>()?;
    for (i, col) in selected_cols.iter().enumerate() {
        print!("{}", col);
        if i < selected_cols.len() - 1 {
            print!("{}", DELIMITER as char);
        }
    }
    println!();
    let filter = condition
        .map(|(col, val)| {
            let idx = headers
                .iter()
                .position(|h| h == col)
                .ok_or_else(|| NeuxError::ColumnNotFound(col.clone(), table.to_string()))?;
            Ok::<_, NeuxError>((idx, val.clone()))
        })
        .transpose()?;
    let mut row_count = 0;
    for row in rows {
        let mut ok = true;
        if let Some((idx, target)) = &filter {
            if &row[*idx] != target {
                ok = false;
            }
        }
        if ok {
            for (i, &idx) in col_indices.iter().enumerate() {
                print!("{}", row[idx]);
                if i < col_indices.len() - 1 {
                    print!("{}", DELIMITER as char);
                }
            }
            println!();
            row_count += 1;
        }
    }
    if row_count == 0 {
        println!("No rows found");
    }
    Ok(())
}

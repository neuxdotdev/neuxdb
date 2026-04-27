use crate::core::storage::read_table;
use crate::core::syntax::{ComparisonOp, WhereClause};
use crate::error::{NeuxDbError, Result};
use crate::types::{Row, Value};
pub fn select_rows(
    table: &str,
    columns: &[String],
    condition: Option<&WhereClause>,
) -> Result<Vec<Row>> {
    let (headers, rows) = read_table(table)?;
    let selected_cols: Vec<String> = if columns.len() == 1 && columns[0] == "*" {
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
    let filtered_rows: Vec<Row> = rows
        .into_iter()
        .filter(|row| match condition {
            Some(cond) => eval_where(row, &headers, cond),
            None => true,
        })
        .map(|row| col_indices.iter().map(|&idx| row[idx].clone()).collect())
        .collect();
    Ok(filtered_rows)
}
fn eval_where(row: &[Value], headers: &[String], clause: &WhereClause) -> bool {
    match clause {
        WhereClause::Condition {
            column,
            operator,
            value,
        } => {
            let idx = headers.iter().position(|h| h == column);
            if idx.is_none() {
                return false;
            }
            let cell = &row[idx.unwrap()];
            match operator {
                ComparisonOp::Eq => cell == value,
                ComparisonOp::Ne => cell != value,
                ComparisonOp::Lt => compare_values(cell, value) == std::cmp::Ordering::Less,
                ComparisonOp::Gt => compare_values(cell, value) == std::cmp::Ordering::Greater,
                ComparisonOp::Le => compare_values(cell, value) != std::cmp::Ordering::Greater,
                ComparisonOp::Ge => compare_values(cell, value) != std::cmp::Ordering::Less,
                ComparisonOp::Like => like_match(&cell.to_like_string(), &value.to_like_string()),
            }
        }
        WhereClause::And(a, b) => eval_where(row, headers, a) && eval_where(row, headers, b),
        WhereClause::Or(a, b) => eval_where(row, headers, a) || eval_where(row, headers, b),
    }
}
fn compare_values(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x.cmp(y),
        (Value::Text(x), Value::Text(y)) => x.cmp(y),
        _ => a.to_string().cmp(&b.to_string()),
    }
}
fn like_match(s: &str, pat: &str) -> bool {
    let mut s_iter = s.chars().peekable();
    let mut p_iter = pat.chars().peekable();
    loop {
        match p_iter.peek() {
            Some('%') => {
                p_iter.next();
                if p_iter.peek().is_none() {
                    return true;
                }
                while s_iter.peek().is_some() {
                    let rest_s: String = s_iter.clone().collect();
                    let rest_p: String = p_iter.clone().collect();
                    if like_match(&rest_s, &rest_p) {
                        return true;
                    }
                    s_iter.next();
                }
                return false;
            }
            Some('_') => {
                p_iter.next();
                if s_iter.next().is_none() {
                    return false;
                }
            }
            Some(&c) => {
                p_iter.next();
                if s_iter.next() != Some(c) {
                    return false;
                }
            }
            None => return s_iter.next().is_none(),
        }
    }
}
